// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CssLocation;
use gio;
use glib;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CssSection(Shared<gtk_sys::GtkCssSection>);

    match fn {
        ref => |ptr| gtk_sys::gtk_css_section_ref(ptr),
        unref => |ptr| gtk_sys::gtk_css_section_unref(ptr),
        get_type => || gtk_sys::gtk_css_section_get_type(),
    }
}

impl CssSection {
    pub fn new<P: IsA<gio::File>>(file: Option<&P>, start: &CssLocation, end: &CssLocation) -> CssSection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_css_section_new(file.map(|p| p.as_ref()).to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_end_location(&self) -> Option<CssLocation> {
        unsafe {
            from_glib_none(gtk_sys::gtk_css_section_get_end_location(self.to_glib_none().0))
        }
    }

    pub fn get_file(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_sys::gtk_css_section_get_file(self.to_glib_none().0))
        }
    }

    pub fn get_parent(&self) -> Option<CssSection> {
        unsafe {
            from_glib_none(gtk_sys::gtk_css_section_get_parent(self.to_glib_none().0))
        }
    }

    pub fn get_start_location(&self) -> Option<CssLocation> {
        unsafe {
            from_glib_none(gtk_sys::gtk_css_section_get_start_location(self.to_glib_none().0))
        }
    }

    pub fn print(&self, string: &mut glib::String) {
        unsafe {
            gtk_sys::gtk_css_section_print(self.to_glib_none().0, string.to_glib_none_mut().0);
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_css_section_to_string(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for CssSection {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
