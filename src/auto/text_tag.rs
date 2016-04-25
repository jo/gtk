// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextTag(Object<ffi::GtkTextTag>);

    match fn {
        get_type => || ffi::gtk_text_tag_get_type(),
    }
}

impl TextTag {
    pub fn new(name: Option<&str>) -> TextTag {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_tag_new(name.to_glib_none().0))
        }
    }

    //pub fn event<T: IsA<glib::Object>>(&self, event_object: &T, event: /*Unknown conversion*//*Unimplemented*/Event, iter: &TextIter) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_tag_event() }
    //}

    pub fn get_priority(&self) -> i32 {
        unsafe {
            ffi::gtk_text_tag_get_priority(self.to_glib_none().0)
        }
    }

    pub fn set_priority(&self, priority: i32) {
        unsafe {
            ffi::gtk_text_tag_set_priority(self.to_glib_none().0, priority);
        }
    }

    //pub fn connect_event<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Unknown conversion event: Gdk.Event
    //}
}
