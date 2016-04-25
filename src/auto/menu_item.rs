// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Container;
use Menu;
use Object;
use Widget;
use ffi;
use ffi::GtkMenuItem;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gpointer;
use libc::c_int;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct MenuItem(Object<ffi::GtkMenuItem>): Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_item_get_type(),
    }
}

impl MenuItem {
    pub fn new() -> MenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> MenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> MenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait MenuItemExt {
    fn deselect(&self);

    fn get_accel_path(&self) -> Option<String>;

    fn get_label(&self) -> Option<String>;

    fn get_reserve_indicator(&self) -> bool;

    fn get_right_justified(&self) -> bool;

    fn get_submenu(&self) -> Option<Widget>;

    fn get_use_underline(&self) -> bool;

    fn select(&self);

    fn set_accel_path(&self, accel_path: Option<&str>);

    fn set_label(&self, label: &str);

    fn set_reserve_indicator(&self, reserve: bool);

    fn set_right_justified(&self, right_justified: bool);

    fn set_submenu<T: IsA<Menu>>(&self, submenu: Option<&T>);

    fn set_use_underline(&self, setting: bool);

    fn toggle_size_allocate(&self, allocation: i32);

    fn toggle_size_request(&self, requisition: &mut i32);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_activate_item<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_deselect<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_select<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_toggle_size_allocate<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    //fn connect_toggle_size_request<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<MenuItem> + IsA<Object>> MenuItemExt for O {
    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_item_deselect(self.to_glib_none().0);
        }
    }

    fn get_accel_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_accel_path(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_label(self.to_glib_none().0))
        }
    }

    fn get_reserve_indicator(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_reserve_indicator(self.to_glib_none().0))
        }
    }

    fn get_right_justified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_right_justified(self.to_glib_none().0))
        }
    }

    fn get_submenu(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_item_get_submenu(self.to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_item_get_use_underline(self.to_glib_none().0))
        }
    }

    fn select(&self) {
        unsafe {
            ffi::gtk_menu_item_select(self.to_glib_none().0);
        }
    }

    fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            ffi::gtk_menu_item_set_accel_path(self.to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_menu_item_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_reserve_indicator(&self, reserve: bool) {
        unsafe {
            ffi::gtk_menu_item_set_reserve_indicator(self.to_glib_none().0, reserve.to_glib());
        }
    }

    fn set_right_justified(&self, right_justified: bool) {
        unsafe {
            ffi::gtk_menu_item_set_right_justified(self.to_glib_none().0, right_justified.to_glib());
        }
    }

    fn set_submenu<T: IsA<Menu>>(&self, submenu: Option<&T>) {
        unsafe {
            ffi::gtk_menu_item_set_submenu(self.to_glib_none().0, submenu.to_glib_none().0);
        }
    }

    fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_menu_item_set_use_underline(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn toggle_size_allocate(&self, allocation: i32) {
        unsafe {
            ffi::gtk_menu_item_toggle_size_allocate(self.to_glib_none().0, allocation);
        }
    }

    fn toggle_size_request(&self, requisition: &mut i32) {
        unsafe {
            ffi::gtk_menu_item_toggle_size_request(self.to_glib_none().0, requisition);
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_activate_item<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-item",
                transmute(activate_item_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_deselect<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deselect",
                transmute(deselect_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_select<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select",
                transmute(select_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_toggle_size_allocate<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-size-allocate",
                transmute(toggle_size_allocate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_toggle_size_request<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unimplemented object: *.Pointer
    //}
}

unsafe extern "C" fn activate_trampoline<T>(this: *mut GtkMenuItem, f: gpointer)
where T: IsA<MenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuItem::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn activate_item_trampoline<T>(this: *mut GtkMenuItem, f: gpointer)
where T: IsA<MenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuItem::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn deselect_trampoline<T>(this: *mut GtkMenuItem, f: gpointer)
where T: IsA<MenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuItem::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn select_trampoline<T>(this: *mut GtkMenuItem, f: gpointer)
where T: IsA<MenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuItem::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn toggle_size_allocate_trampoline<T>(this: *mut GtkMenuItem, object: c_int, f: gpointer)
where T: IsA<MenuItem> {
    callback_guard!();
    let f: &Box_<Fn(&T, i32) + 'static> = transmute(f);
    f(&MenuItem::from_glib_none(this).downcast_unchecked(), object)
}
