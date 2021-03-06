// https://askubuntu.com/questions/98692/how-to-add-support-for-the-global-menu-to-a-python-non-gtk-non-qt-app

extern crate dbus;

/// Interface to the DBUS menu registering API  on Linux
pub mod dbusmenu;
/// Interface to the Window menu registering API on Linux
pub mod appmenuregistrar;
/// Menu structure for abstracting the menu registering / changes
pub mod menu;

pub use dbusmenu::*;
pub use menu::*;
pub use appmenuregistrar::*;
