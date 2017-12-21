extern crate dbusmenu_rs;
extern crate dbus;
extern crate x11_dl;

use std::ffi::CString;
const GL_TRUE: i32 = 1;

const GL_DEPTH_TEST: GLenum = 0x0B71;
const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;

type GLenum =       u32;
type GLbitfield =   u32;
type GLint =        i32;
type GLsizei =      i32;
type GLfloat =      f32;


use dbus::{Connection, BusType};
use dbus::tree::Factory;
use dbusmenu_rs::*;

use std::sync::atomic::*;
use std::sync::{Arc};

#[link(kind = "dylib", name = "GL")]
extern {
    fn glEnable(cap: GLenum) -> ();
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> ();
    fn glClear(mask: GLbitfield) -> ();
}

fn main() {

    let menu_object_path = "/the/path/you/tell/appmenu/registrar";
    let dbus_path_name: dbus::Path = menu_object_path.into();

    let quit = std::sync::Arc::new(AtomicBool::new(false));
    let quit2 = quit.clone();

    let c = Connection::get_private(BusType::Session).unwrap();
    let connection_id = c.unique_name();
    println!("the name of the connection is: {:?}", connection_id);

    let conn = Arc::new(c);
    let ref_conn = conn.clone();

    let _ = std::thread::spawn(move || {

        // Create the menu
        let menu = Menu::new();
        // Notice: Factory created with MData
        let f = Factory::new_fn::<MData>();
        // Create the tree for the interface (get / set, etc. etc.)
        let i1 = com_canonical_dbusmenu_server(&f, (), |minfo| minfo.path.get_data());
        // Add the menu to the objectpath
        let path = f.object_path(menu_object_path, menu).add(i1).introspectable();
        // Start the server with the given interface at the object path
        let t = f.tree(()).add(path);
        t.set_registered(&ref_conn, true).unwrap();

        quit2.store(true, Ordering::SeqCst);

        for e in t.run(&ref_conn, ref_conn.iter(100)) { println!("running: {:?}", e);}
    });

    // Anything but the connection is not used, so it doesn't matter what's passed in here
    // let c2 = Connection::get_private(BusType::Session).unwrap();
    // The ID of the current connection
    // let connection_id = c2.unique_name();

    // wait for thread to start up
    while !quit.load(Ordering::SeqCst) { ::std::thread::sleep(::std::time::Duration::from_millis(2)); }

    let conn_path = dbus::ConnPath {
        conn: &*conn,
        dest: connection_id.into(),
        path: menu_object_path.into(),
        timeout: 100,
    };

    // Tell the appmenu registrar to register the window

    /* ---------------------- X11 -------------------------------- */

    let xlib = match x11_dl::xlib::Xlib::open() {
        Ok(x) => x,
        Err(xerr) => panic!("Error: {}", xerr.detail()),
    };

    let glx_ext = match x11_dl::glx::Glx::open() {
        Ok(ext) => ext,
        Err(xerr) => panic!("Error: {}", xerr.detail()),
    };

    let (display, window, glc) = get_x11_window(&xlib);
    let window_id = window as u32;
    let response = conn_path.register_window(window_id, dbus_path_name);

    println!("response: {:?}", response);
    println!("window ID: {:?}", window_id);
    println!("[main] registered, now sleeping...");

    let mut cur_xevent = x11_dl::xlib::XEvent { pad: [0;24] };
    let mut cur_window_attributes: x11_dl::xlib::XWindowAttributes = unsafe { std::mem::uninitialized() };

    loop {

        unsafe { (xlib.XNextEvent)(display, &mut cur_xevent) };

        let cur_event_type = cur_xevent.get_type();

        match cur_event_type {
            x11_dl::xlib::Expose => {
                unsafe { (xlib.XGetWindowAttributes)(display, window, &mut cur_window_attributes) };
                unsafe { glViewport(0, 0, cur_window_attributes.width, cur_window_attributes.height) };

                /* do drawing here */
                unsafe { glClearColor(1.0, 1.0, 1.0, 0.0) };
                unsafe { glClear(GL_COLOR_BUFFER_BIT) };

                unsafe { (glx_ext.glXSwapBuffers)(display, window) };
            },
            x11_dl::xlib::KeyPress => {
                unsafe { (glx_ext.glXMakeCurrent)(display, 0 /* None ? */, ::std::ptr::null_mut()) };
                unsafe { (glx_ext.glXDestroyContext)(display, glc) };
                unsafe { (xlib.XDestroyWindow)(display, window) };
                unsafe { (xlib.XCloseDisplay)(display) };
                break;
            },
            _ => { },
        }
    }

    // sleeping to see introspection
    ::std::thread::sleep(::std::time::Duration::from_secs(100));
}

fn get_x11_window(xlib: &x11_dl::xlib::Xlib) -> (&mut x11_dl::xlib::Display, x11_dl::xlib::Window, x11_dl::glx::GLXContext) {

    let display_int = 0_i8;
    let dpy = unsafe { (xlib.XOpenDisplay)(&display_int) };

    let display = {
        if dpy.is_null() {
            panic!("Error opening connection to X Server!");
        } else {
            unsafe { &mut*dpy }
        }
    };

    // get root window
    let root = unsafe { (xlib.XDefaultRootWindow)(display) };

    let glx_ext = match x11_dl::glx::Glx::open() {
        Ok(ext) => ext,
        Err(xerr) => panic!("Error: {}", xerr.detail()),
    };

    let mut att = [x11_dl::glx::GLX_RGBA, x11_dl::glx::GLX_DEPTH_SIZE, 24, x11_dl::glx::GLX_DOUBLEBUFFER, x11_dl::glx::GLX_NONE];

    let vi = unsafe { (glx_ext.glXChooseVisual)(dpy, 0, &mut att[0]) };

    let visual_info = { if vi.is_null() {
            panic!("Display does not meet minimum requirements: RGBA buffer, 24-bit depth, double-buffered display");
        } else {
            unsafe { &mut*vi }
        }
    };

    let cmap = unsafe { (xlib.XCreateColormap)(display, root, visual_info.visual, x11_dl::xlib::AllocNone) };

    let mut window_attributes: x11_dl::xlib::XSetWindowAttributes = unsafe { std::mem::uninitialized() };
    window_attributes.event_mask = x11_dl::xlib::ExposureMask | x11_dl::xlib::KeyPressMask;
    window_attributes.colormap = cmap;

    // construct window
    let window = unsafe { (xlib.XCreateWindow)(display, root, 0, 0, 600, 600, 0, visual_info.depth,
                                            1 /* InputOutput */, visual_info.visual,
                                            x11_dl::xlib::CWColormap | x11_dl::xlib::CWEventMask,
                                            &mut window_attributes) };

    let window_title = CString::new("illustrate!").unwrap();

    // show window
    unsafe { (xlib.XMapWindow)(display, window) };
    unsafe { (xlib.XStoreName)(display, window, window_title.as_ptr()) };

    let glc = unsafe { (glx_ext.glXCreateContext)(display, &mut *visual_info, ::std::ptr::null_mut(), GL_TRUE) };
    unsafe { (glx_ext.glXMakeCurrent)(display, window, glc) };
    unsafe { glEnable(GL_DEPTH_TEST) };

    // todo: poll events?

    // todo: setup opengl 3.1 or 3.3

    (display, window, glc)
}
