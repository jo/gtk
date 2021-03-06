// This file was generated by gir (4d68d19) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use Container;
use Object;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ToggleButton(Object<ffi::GtkToggleButton>): Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_toggle_button_get_type(),
    }
}

impl ToggleButton {
    pub fn new() -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToggleButtonExt {
    fn get_active(&self) -> bool;

    fn get_inconsistent(&self) -> bool;

    fn get_mode(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn set_inconsistent(&self, setting: bool);

    fn set_mode(&self, draw_indicator: bool);

    fn toggled(&self);

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ToggleButton> + IsA<Object>> ToggleButtonExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_active(self.to_glib_none().0))
        }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_inconsistent(self.to_glib_none().0))
        }
    }

    fn get_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_mode(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_inconsistent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_mode(&self, draw_indicator: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_mode(self.to_glib_none().0, draw_indicator.to_glib());
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_toggle_button_toggled(self.to_glib_none().0);
        }
    }

    fn connect_toggled<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggled",
                transmute(toggled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn toggled_trampoline<T>(this: *mut ffi::GtkToggleButton, f: glib_ffi::gpointer)
where T: IsA<ToggleButton> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&ToggleButton::from_glib_none(this).downcast_unchecked())
}
