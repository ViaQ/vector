#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type BackPressedEventArgs = *mut ::core::ffi::c_void;
pub type CameraEventArgs = *mut ::core::ffi::c_void;
