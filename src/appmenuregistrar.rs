// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

//! An interface to register menus that are associated with a window in an application.

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

/// An interface to register a menu from an application's window to be displayed in another
/// window.  This manages that association between XWindow Window IDs and the dbus
/// address and object that provides the menu using the dbusmenu dbus interface.
pub trait ComCanonicalAppMenuRegistrar {

    type Err;

    /// Associates a dbusmenu with a window
    ///
    /// **NOTE:** this method assumes that the connection from the caller is the DBus connection
    /// to use for the object.  Applications that use multiple DBus connections will need to
    /// ensure this method is called with the same connection that implmenets the object.
    ///
    /// ### Inputs
    ///
    /// - `window_id`: The XWindow ID of the window
    /// - `menu_object_path`: The object on the dbus interface implementing the dbusmenu interface
    ///
    fn register_window(&self, window_id: u32, menu_object_path: dbus::Path) -> Result<(), Self::Err>;

    /// A method to allow removing a window from the database.  Windows will also be removed
    /// when the client drops off DBus so this is not required.  It is polite though.  And
    /// important for testing.
    ///
    /// ### Inputs
    ///
    /// - `window_id`: The XWindow ID of the window
    ///
    fn unregister_window(&self, window_id: u32) -> Result<(), Self::Err>;

    /// Gets the registered menu for a given window ID.
    ///
    /// ### Inputs
    ///
    /// - `window_id`: The XWindow ID of the window
    ///
    /// ### Outputs
    ///
    /// - `menu_object_path: String`: The path to the object which implements the com.canonical.dbusmenu interface.
    ///
    fn get_menu_for_window(&self, window_id: u32) -> Result<(String, dbus::Path<'static>), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> ComCanonicalAppMenuRegistrar for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn register_window(&self, window_id: u32, menu_object_path: dbus::Path) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"com.canonical.AppMenu.Registrar".into(), &"RegisterWindow".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(window_id);
            i.append(menu_object_path);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn unregister_window(&self, window_id: u32) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"com.canonical.AppMenu.Registrar".into(), &"UnregisterWindow".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(window_id);
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_menu_for_window(&self, window_id: u32) -> Result<(String, dbus::Path<'static>), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"com.canonical.AppMenu.Registrar".into(), &"GetMenuForWindow".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(window_id);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let service: String = try!(i.read());
        let menu_object_path: dbus::Path<'static> = try!(i.read());
        Ok((service, menu_object_path))
    }
}

/// Returns a tree that implements the `com.canonical.AppMenu.Registrar` interface
/// From this tree, you can start a server
///
/// Only useful if you are writing a replacement for the Registrar
///
pub fn com_canonical_app_menu_registrar_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where D: tree::DataType, D::Method: Default, T: ComCanonicalAppMenuRegistrar<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T {
    let i = factory.interface("com.canonical.AppMenu.Registrar", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let window_id: u32 = try!(i.read());
        let menu_object_path: dbus::Path = try!(i.read());
        let d = fclone(minfo);
        try!(d.register_window(window_id, menu_object_path));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("RegisterWindow", Default::default(), h);
    let m = m.in_arg(("windowId", "u"));
    let m = m.in_arg(("menuObjectPath", "o"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let window_id: u32 = try!(i.read());
        let d = fclone(minfo);
        try!(d.unregister_window(window_id));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("UnregisterWindow", Default::default(), h);
    let m = m.in_arg(("windowId", "u"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let window_id: u32 = try!(i.read());
        let d = fclone(minfo);
        let (service, menu_object_path) = try!(d.get_menu_for_window(window_id));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(service);
        let rm = rm.append1(menu_object_path);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMenuForWindow", Default::default(), h);
    let m = m.in_arg(("windowId", "u"));
    let m = m.out_arg(("service", "s"));
    let m = m.out_arg(("menuObjectPath", "o"));
    let i = i.add_m(m);
    i
}
