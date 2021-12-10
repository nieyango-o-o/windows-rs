#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OOBEComplete(::core::mem::transmute(isoobecomplete)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterWaitUntilOOBECompleted(::core::mem::transmute(oobecompletedcallback), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(waithandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterWaitUntilOOBECompleted(::core::mem::transmute(waithandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
