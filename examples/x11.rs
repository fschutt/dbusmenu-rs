extern crate dbusmenu_rs;
extern crate dbus;

use dbus::{Connection, BusType};
use dbus::tree::Factory;
use dbusmenu_rs::*;

fn main() {
    // let c = Connection::get_private(BusType::Session);
    let f = Factory::new_fn::<()>();
    let menu = Menu::new();
    let i1 = com_canonical_dbusmenu_server(&f, (), move |_| &menu);
}    // let join_handle = ::std::thread::spawn(|| { });

