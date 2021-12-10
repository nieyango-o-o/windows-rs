#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemInterop(::windows::core::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, window: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateForMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Gdi::HMONITOR>, T: ::windows::core::Interface>(&self, monitor: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), monitor.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: IGraphicsCaptureItemInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsCaptureItemInterop> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsCaptureItemInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGraphicsCaptureItemInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGraphicsCaptureItemInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsCaptureItemInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsCaptureItemInterop {}
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
