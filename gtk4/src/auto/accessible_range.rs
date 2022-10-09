// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Accessible;
use glib::object::IsA;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkAccessibleRange")]
    pub struct AccessibleRange(Interface<ffi::GtkAccessibleRange, ffi::GtkAccessibleRangeInterface>) @requires Accessible;

    match fn {
        type_ => || ffi::gtk_accessible_range_get_type(),
    }
}

impl AccessibleRange {
    pub const NONE: Option<&'static AccessibleRange> = None;
}

pub trait AccessibleRangeExt: 'static {}

impl<O: IsA<AccessibleRange>> AccessibleRangeExt for O {}

impl fmt::Display for AccessibleRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AccessibleRange")
    }
}
