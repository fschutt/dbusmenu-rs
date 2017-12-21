//! Menu abstrction module

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use dbusmenu::ComCanonicalDbusmenu;
use dbus::arg;
use dbus;

#[derive(Default)]
pub struct Menu {
    /// - `revision: i32`: The revision number of the layout.
    /// For matching with layoutUpdated signals.
    revision: Rc<RefCell<i32>>,
    /// The window ID that the menu was created on
    pub window_id: Option<u32>,
    /// The actual Menu structure, indexed by their action name / identifier
    pub menu: HashMap<&'static str, SubMenu>,
    /// The current language.
    /// **NOTE** : The default is "en", so make sure to have at least one
    /// entry in the menu items labels that is indexed by "en"
    pub cur_language: &'static str,
}

/// Top-level submenu. Not to be confused with MenuData::SubMenuItem
pub struct SubMenu {
    /// The label of the menu
    pub label: HashMap<String, String>,
    /// The menu items, indexed by their action name
    pub menu: HashMap<String, MenuItem>,
}

impl Menu {

    /// Creates a new window, but doesn't add it to any window yet
    /// Starts a new thread for maintaining the rendering loop
    pub fn new() -> Self {
        Self {
            revision: Rc::new(RefCell::new(0)),
            window_id: None,
            menu: HashMap::new(),
            cur_language: "en",
        }
    }

    /// Adds the menu to the window - takes XID of window as parameter
    pub fn add_to_window(&mut self, window_id: u32) {
        self.window_id = Some(window_id);
        // todo: notify app menu registrar here
        println!("registered window!");
    }

    /// Removes the menu
    pub fn remove_from_window(&mut self) {
        self.window_id = None;
        // appmenu unregister window
        // should also be called on drop
        println!("unregistered window!");
    }

    /// Removes an item from the menu list.
    /// Does not error out, but rather returns if the removal was successful
    pub fn remove_item<S: Into<String>>(item: S) -> bool {
        let item_id = item.into();
        println!("remove_item: {:?}", item_id);
        false
    }

    /// Adds an item to the menu list.
    /// Does not error out, but rather returns if the add was successful
    pub fn add_item<S: Into<String>>(item: S) -> bool {
        let item_id = item.into();
        println!("add item: {:?}", item_id);
        false
    }

    /// Actually constructs the window so that it shows the menu now
    /// Sends the menu over DBus
    pub fn show() {

    }
}

pub enum MenuItem {
    /// Text menu item, regular. Gets called if clicked
    TextMenuItem(MenuData<Box<Fn() -> ()>>),
    /// Checkbox menu item,
    CheckboxMenuItem(MenuData<Box<Fn(bool) -> ()>>),
    /// Radio menu item, consisting of multiple menu items.
    /// Callback gets a string of the currently selected value
    RadioMenuItem(Vec<MenuData<Box<Fn(String) -> ()>>>),
    /// Seperator
    Seperator(),
    /// Submenu
    SubMenuItem(String, Box<SubMenu>),
}

#[derive(Debug)]
pub struct MenuData<F> {
    /// The action to execute, depends on the type of menu item
    pub action: F,
    /// Optional image as PNG bytes
    pub image: Option<Vec<u8>>,
    /// The label(s) of the menu item, indexed by language identifier
    ///
    /// For example:
    ///
    /// de - Datei öffnen
    /// en - Open file
    pub label: HashMap<String, String>,
    /// Should the menu entry be activated on hovering
    pub activate_on_hover: bool,
    /// Optional shortcuts in the format of a string
    /// `[["Control", "S"]]`
    /// `[["Control", "Q"], ["Alt", "X"]]`
    /// This is only a visual cue (todo: really?)
    pub shortcut: Option<Vec<ShortcutData>>,
}

#[derive(Debug, Clone)]
pub enum ShortcutData {
    /// The "Control" in CTRL + S
    ControlChar(CtrlChar),
    /// The "S" in CTRL + S
    Char(String),
}

/// The four controls registered by dbus
#[derive(Debug, Copy, Clone)]
pub enum CtrlChar {
    Ctrl,
    Alt,
    Shift,
    Super,
}

/*
    0 => [
        "type" => "standard" | "seperator",
        "label" => "Hello",
        "enabled" => true,
        "visible" => true,
        "icon-name" => "hello.png",
        "icon-data" => Vec<u8>,
        "shortcut" => [["Control", "S"]],
        "toggle-type" => "checkmark" | "radio", "",
        "toggle-state" => MenuItemToggleState,
        "children-display" => "" | "submenu",
    ],

    defaults:

    type = "standard",
    label = "",
    enabled = "",
    visible = "",
    icon-name = "",
    icon-data = None,
    shortcut = None,
    toggle-type = "",
    toggle-state = -1
    children-display = "",
*/

#[derive(Debug)]
pub enum MenuItemToggleState {
    On,
    Off,
    Invalid,
}

impl Into<i32> for MenuItemToggleState {
    fn into(self) -> i32 {
        match self {
            MenuItemToggleState::On => 1,
            MenuItemToggleState::Off => 0,
            MenuItemToggleState::Invalid => -1,
        }
    }
}

/// Implement the ComCanonicalMenu so we can push it to the server
impl ComCanonicalDbusmenu for Menu {

    type Err = dbus::tree::MethodErr;

    /// - `parent_id`: The ID of the parent node for the layout.  For grabbing the layout from the root node use zero.
    /// - `recursion_depth`: The amount of levels of recursion to use.  This affects the content of the second variant array.
    ///      - -1: deliver all the items under the @a parentId.
    ///      - 0: no recursion, the array will be empty.
    ///      - n: array will contains items up to 'n' level depth.
    /// - `property_names`: The list of item properties we are interested in. If there are no entries in the list all of the properties will be sent.
    ///
    /// ### Outputs
    ///
    /// - `revision: i32`: The revision number of the layout.  For matching with layoutUpdated signals.
    /// - `layout: HashMap`: The layout, as a recursive structure.
    ///
    fn get_layout(&self, parent_id: i32, recursion_depth: i32, property_names: Vec<&str>)
    -> Result<(u32, (i32, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Vec<arg::Variant<Box<arg::RefArg>>>)), Self::Err> {

        // I have no idea if this will actually work in any way possible
        // (u, (ia{sv}av))

        // Nautilus: 0, 2, []
        // Answer: 14

        /*
            try!(m.as_result());
            let mut i = m.iter_init();
            let revision: u32 = try!(i.read());
            let layout: (i32, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Vec<arg::Variant<Box<arg::RefArg>>>) = try!(i.read());
            Ok((revision, layout))
        */

        use dbus::Message;
        use dbus::Member;

        println!("getlayout called!");
        let mut m = Message::new_method_call("com.canonical.dbusmenu", "com/canonical/dbusmenu", "com.canonical.dbusmenu", Member::new("com.canonical.dbusmenu".as_bytes()).unwrap()).unwrap();
        try!(m.as_result());
        let mut i = m.iter_init();

        let mut map = HashMap::<String, arg::Variant<Box<arg::RefArg>>>::new();
        map.insert("data-hello".into(), arg::Variant::new_refarg(&mut i).unwrap());
        *self.revision.borrow_mut() += 1;
        Ok((1, (*self.revision.borrow(), map, Vec::new())))
    }

    fn get_group_properties(&self, ids: Vec<i32>, property_names: Vec<&str>)
    -> Result<Vec<(i32, ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>)>, Self::Err> {
        // I AM NOT SURE IF THS WORKS!
        println!("get_group_properties called: {:?}, {:?}", ids, property_names);
/*
    method call time=1510750424.121891
    sender=:1.318
    -> destination=org.freedesktop.DBus
    serial=1 path=/org/freedesktop/DBus;
    interface=org.freedesktop.DBus;
    member=Hello
*/

    // warning: other method is also called "hello"
    // If Nautilus is called with [0], returns [(0, {'children-display': 'submenu'})]
        let mut properties_hashmap = HashMap::<String, arg::Variant<Box<arg::RefArg>>>::new();
        properties_hashmap.insert("label".into(), arg::Variant(Box::new("Hello".to_string())));
        Ok(vec![(0, properties_hashmap)])
    }

    fn get_property(&self, id: i32, name: &str) -> Result<arg::Variant<Box<arg::RefArg>>, Self::Err> {
        println!("get property called!");
        // Nautilus get_propery(0, 'children-display') -> 'submenu'
        Ok(arg::Variant(Box::new("everything is OK".to_string())))
    }

    fn event(&self, id: i32, event_id: &str, data: arg::Variant<Box<arg::RefArg>>, timestamp: u32) -> Result<(), Self::Err> {
        println!("event called!");

        if event_id == "clicked" {
            println!("received clicked event for menu item {:?}", id);
        } else if event_id == "hovered" {
            println!("received hovered event for menu item {:?}", id);
        }

        Ok(())
    }

    fn about_to_show(&self, id: i32) -> Result<bool, Self::Err> {
        // ??? "Whether this AboutToShow event should result in the menu being updated."
        // not sure what this means
        println!("about_to_show called, id: {:?}", id);
        Ok(true)
    }

    fn get_version(&self) -> Result<u32, Self::Err> {
        // ????
        println!("about_to_show called!");

        Ok(3)
    }

    fn get_status(&self) -> Result<String, Self::Err> {
       println!("get_status called!");

       // Menus will always be in "normal" state, may change later on
        Ok("normal".into())
    }
}

#[derive(Default, Clone)]
pub struct MData;

impl<'a> dbus::tree::DataType for MData {
    type Tree = ();
    type ObjectPath = Menu; // Every objectpath in the tree now owns a menu object.
    type Property = ();
    type Interface = ();
    type Method = ();
    type Signal = ();
}

/// Since parts of the menu are not printable, implement Debug trait manually
/// Needed because of a bug in rust: https://github.com/rust-lang/rust/issues/31518
impl ::std::fmt::Debug for Menu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Menu {{ /* non-printable fields omitted */ }}")
    }
}
