//! Menu abstrction module

use std::collections::HashMap;
use std::thread::JoinHandle;

pub struct Menu<T> {
    /// The window ID that the menu was created on
    pub window_id: Option<u32>,
    /// The background thread on which the menu is listening
    pub thread_handle: Option<JoinHandle<T>>,
    /// The actual Menu structure, indexed by their action name / identifier
    pub menu: HashMap<String, MenuItem<T>>,
    /// The current language.
    /// **NOTE** : The default is "en", so make sure to have at least one
    /// entry in the menu items labels that is indexed by "en"
    pub cur_language: String,
}

impl<T> Menu<T> {

    /// Creates a new window, but doesn't add it to any window yet
    /// Starts a new thread for maintaining the rendering loop
    pub fn new() -> Self {
        Self {
            window_id: None,
            thread_handle: None,
            menu: HashMap::new(),
            cur_language: "en".into(),
        }
    }

    /// Adds the menu to the window - takes XID of window as parameter
    pub fn add_to_window(window_id: u32) {

    }

    /// Removes the menu
    pub fn remove_from_window(window_id: u32) {

    }

    /// Removes an item from the menu list.
    /// Does not error out, but rather returns if the removal was successful
    pub fn remove_item(window_id: u32) -> bool {
        false
    }
}

pub enum MenuItem<T> {
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
    SubMenuItem(String, Box<Menu<T>>),
}

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

pub enum ShortcutData {
    /// The "Control" in CTRL + S
    ControlChar(CtrlChar),
    /// The "S" in CTRL + S
    Char(String),
}

/// The four controls registered by dbus
pub enum CtrlChar {
    Ctrl,
    Alt,
    Shift,
    Super,
}
