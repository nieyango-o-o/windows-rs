// Bindings generated by `windows-bindgen` 0.52.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
::windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : HSTRING, instance : *mut IInspectable) -> HRESULT);
pub type HRESULT = i32;
pub type HSTRING = *mut ::core::ffi::c_void;
pub type IInspectable = *mut ::core::ffi::c_void;
