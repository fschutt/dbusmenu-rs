// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

#![allow(dead_code)]
use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgGtkMenus {
    type Err;
    fn start(&self, input_group: Vec<u32>) -> Result<(u32, u32, Vec<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>>), Self::Err>;
    fn end(&self, input_group: Vec<u32>) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgGtkMenus for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn start(&self, input_group: Vec<u32>) -> Result<(u32, u32, Vec<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>>), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.gtk.Menus".into(), &"Start".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(input_group);
        }));
        try!(m.as_result());
        let mut i = m.iter_init();
        let subscription_group: u32 = try!(i.read());
        let menu_id: u32 = try!(i.read());
        let menu_items: Vec<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>> = try!(i.read());
        Ok((subscription_group, menu_id, menu_items))
    }

    fn end(&self, input_group: Vec<u32>) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.gtk.Menus".into(), &"End".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(input_group);
        }));
        try!(m.as_result());
        Ok(())
    }
}

pub fn org_gtk_menus_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where D: tree::DataType, D::Method: Default, T: OrgGtkMenus<Err=tree::MethodErr>,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T {
    let i = factory.interface("org.gtk.Menus", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let input_group: Vec<u32> = try!(i.read());
        let d = fclone(minfo);
        let (subscription_group, menu_id, menu_items) = try!(d.start(input_group));
        let rm = minfo.msg.method_return();
        let rm = rm.append1(subscription_group);
        let rm = rm.append1(menu_id);
        let rm = rm.append1(menu_items);
        Ok(vec!(rm))
    };
    let m = factory.method("Start", Default::default(), h);
    let m = m.in_arg(("inputGroup", "au"));
    let m = m.out_arg(("subscriptionGroup", "u"));
    let m = m.out_arg(("menuId", "u"));
    let m = m.out_arg(("menuItems", "aa{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let input_group: Vec<u32> = try!(i.read());
        let d = fclone(minfo);
        try!(d.end(input_group));
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("End", Default::default(), h);
    let m = m.in_arg(("inputGroup", "au"));
    let i = i.add_m(m);
    i
}

pub fn org_gtk_menus_changed_emit<C: ::std::ops::Deref<Target=dbus::Connection>>(conn: &dbus::ConnPath<C>, input: Vec<(u32, u32, u32, u32, Vec<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>>)>) -> Result<(), dbus::Error> {
    conn.signal_with_args(&"org.gtk.Menus".into(), &"Changed".into(), move |msg| {
         let mut i = arg::IterAppend::new(msg);
         i.append(input);
    }).map(|_| ())
}
