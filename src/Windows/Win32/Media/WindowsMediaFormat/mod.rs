#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows::core::HRESULT,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AM_WMT_EVENT_DATA {}
impl ::core::clone::Clone for AM_WMT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AM_WMT_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AM_WMT_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for AM_WMT_EVENT_DATA {}
impl ::core::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_ClientNetManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd12a3ce_9c42_11d2_beed_0060082f2054);
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[repr(C)]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl ::core::marker::Copy for DRM_COPY_OPL {}
impl ::core::clone::Clone for DRM_COPY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_COPY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_COPY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_COPY_OPL {}
impl ::core::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl ::core::marker::Copy for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::clone::Clone for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for DRM_OPL_OUTPUT_IDS {}
impl ::core::clone::Clone for DRM_OPL_OUTPUT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_OPL_OUTPUT_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OPL_OUTPUT_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
impl ::core::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DRM_OPL_TYPES: u32 = 1u32;
#[repr(C)]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows::core::GUID,
    pub bConfigData: u8,
}
impl ::core::marker::Copy for DRM_OUTPUT_PROTECTION {}
impl ::core::clone::Clone for DRM_OUTPUT_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_OUTPUT_PROTECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OUTPUT_PROTECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OUTPUT_PROTECTION {}
impl ::core::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl ::core::marker::Copy for DRM_PLAY_OPL {}
impl ::core::clone::Clone for DRM_PLAY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_PLAY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_PLAY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_PLAY_OPL {}
impl ::core::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DRM_VAL16 {}
impl ::core::clone::Clone for DRM_VAL16 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_VAL16 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VAL16>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VAL16 {}
impl ::core::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl ::core::marker::Copy for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::clone::Clone for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VIDEO_OUTPUT_PROTECTION_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IAMWMBufferPass(::windows::core::IUnknown);
impl IAMWMBufferPass {
    pub unsafe fn SetNotify<'a, Param0: ::windows::core::IntoParam<'a, IAMWMBufferPassCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAMWMBufferPass> for ::windows::core::IUnknown {
    fn from(value: IAMWMBufferPass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAMWMBufferPass> for ::windows::core::IUnknown {
    fn from(value: &IAMWMBufferPass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAMWMBufferPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAMWMBufferPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAMWMBufferPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAMWMBufferPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAMWMBufferPass {}
unsafe impl ::windows::core::Interface for IAMWMBufferPass {
    type Vtable = IAMWMBufferPassVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dd816d7_e740_4123_9e24_2444412644d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPassVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IAMWMBufferPassCallback(::windows::core::IUnknown);
impl IAMWMBufferPassCallback {
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer3>, Param1: ::windows::core::IntoParam<'a, super::DirectShow::IPin>>(&self, pnssbuffer3: Param0, ppin: Param1, prtstart: *const i64, prtend: *const i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnssbuffer3.into_param().abi(), ppin.into_param().abi(), ::core::mem::transmute(prtstart), ::core::mem::transmute(prtend)).ok()
    }
}
impl ::core::convert::From<IAMWMBufferPassCallback> for ::windows::core::IUnknown {
    fn from(value: IAMWMBufferPassCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAMWMBufferPassCallback> for ::windows::core::IUnknown {
    fn from(value: &IAMWMBufferPassCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAMWMBufferPassCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAMWMBufferPassCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAMWMBufferPassCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAMWMBufferPassCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAMWMBufferPassCallback {}
unsafe impl ::windows::core::Interface for IAMWMBufferPassCallback {
    type Vtable = IAMWMBufferPassCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb25b8372_d2d2_44b2_8653_1b8dae332489);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPassCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Media_DirectShow")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))] usize,
);
#[repr(transparent)]
pub struct INSNetSourceCreator(::windows::core::IUnknown);
impl INSNetSourceCreator {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNetSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszstreamname: Param0, pmonitor: Param1, pdata: *const u8, pusercontext: Param3, pcallback: Param4, qwcontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), pmonitor.into_param().abi(), ::core::mem::transmute(pdata), pusercontext.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(qwcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetSourceProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstreamname: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNetSourceAdminInterface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszstreamname: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<INSNetSourceCreator> for ::windows::core::IUnknown {
    fn from(value: INSNetSourceCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSNetSourceCreator> for ::windows::core::IUnknown {
    fn from(value: &INSNetSourceCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSNetSourceCreator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INSNetSourceCreator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSNetSourceCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSNetSourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSNetSourceCreator {}
unsafe impl ::windows::core::Interface for INSNetSourceCreator {
    type Vtable = INSNetSourceCreatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c0e4080_9081_11d2_beec_0060082f2054);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSNetSourceCreatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct INSSBuffer(::windows::core::IUnknown);
impl INSSBuffer {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
}
impl ::core::convert::From<INSSBuffer> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INSSBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer {}
unsafe impl ::windows::core::Interface for INSSBuffer {
    type Vtable = INSSBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct INSSBuffer2(::windows::core::IUnknown);
impl INSSBuffer2 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
}
impl ::core::convert::From<INSSBuffer2> for INSSBuffer {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for INSSBuffer {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer2> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer2 {}
unsafe impl ::windows::core::Interface for INSSBuffer2 {
    type Vtable = INSSBuffer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f528693_1035_43fe_b428_757561ad3a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct INSSBuffer3(::windows::core::IUnknown);
impl INSSBuffer3 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer2 {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer2 {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for &INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer3 {}
unsafe impl ::windows::core::Interface for INSSBuffer3 {
    type Vtable = INSSBuffer3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct INSSBuffer4(::windows::core::IUnknown);
impl INSSBuffer4 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbufferpropertyindex), ::core::mem::transmute(pguidbufferproperty), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer3 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer3 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer3> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer3> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer2 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer2 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer4 {}
unsafe impl ::windows::core::Interface for INSSBuffer4 {
    type Vtable = INSSBuffer4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMAddressAccess(::windows::core::IUnknown);
impl IWMAddressAccess {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: WM_ADDRESS_ACCESSENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess> for ::windows::core::IUnknown {
    fn from(value: IWMAddressAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess> for ::windows::core::IUnknown {
    fn from(value: &IWMAddressAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAddressAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMAddressAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess {}
unsafe impl ::windows::core::Interface for IWMAddressAccess {
    type Vtable = IWMAddressAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMAddressAccess2(::windows::core::IUnknown);
impl IWMAddressAccess2 {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: WM_ADDRESS_ACCESSENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(pbstraddress), ::core::mem::transmute(pbstrmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAccessEntryEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, aetype: WM_AETYPE, bstraddress: Param1, bstrmask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), bstraddress.into_param().abi(), bstrmask.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAddressAccess> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAddressAccess> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAddressAccess> for &IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAddressAccess> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMAddressAccess2> for ::windows::core::IUnknown {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for ::windows::core::IUnknown {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess2 {}
unsafe impl ::windows::core::Interface for IWMAddressAccess2 {
    type Vtable = IWMAddressAccess2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMAuthorizer(::windows::core::IUnknown);
impl IWMAuthorizer {
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<IWMAuthorizer> for ::windows::core::IUnknown {
    fn from(value: IWMAuthorizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAuthorizer> for ::windows::core::IUnknown {
    fn from(value: &IWMAuthorizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAuthorizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMAuthorizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAuthorizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAuthorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAuthorizer {}
unsafe impl ::windows::core::Interface for IWMAuthorizer {
    type Vtable = IWMAuthorizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b67d36_a9ad_4eb4_baef_db284ef5504c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAuthorizerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMBackupRestoreProps(::windows::core::IUnknown);
impl IWMBackupRestoreProps {
    pub unsafe fn GetPropCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveProp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pcwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pcwszname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllProps(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMBackupRestoreProps> for ::windows::core::IUnknown {
    fn from(value: IWMBackupRestoreProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBackupRestoreProps> for ::windows::core::IUnknown {
    fn from(value: &IWMBackupRestoreProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMBackupRestoreProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMBackupRestoreProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBackupRestoreProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBackupRestoreProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBackupRestoreProps {}
unsafe impl ::windows::core::Interface for IWMBackupRestoreProps {
    type Vtable = IWMBackupRestorePropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestorePropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMBandwidthSharing(::windows::core::IUnknown);
impl IWMBandwidthSharing {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate), ::core::mem::transmute(pmsbufferwindow)).ok()
    }
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbitrate), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for IWMStreamList {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for IWMStreamList {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for ::windows::core::IUnknown {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for ::windows::core::IUnknown {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBandwidthSharing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBandwidthSharing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBandwidthSharing {}
unsafe impl ::windows::core::Interface for IWMBandwidthSharing {
    type Vtable = IWMBandwidthSharingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMClientConnections(::windows::core::IUnknown);
impl IWMClientConnections {
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: WM_CLIENT_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
}
impl ::core::convert::From<IWMClientConnections> for ::windows::core::IUnknown {
    fn from(value: IWMClientConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections> for ::windows::core::IUnknown {
    fn from(value: &IWMClientConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMClientConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMClientConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections {}
unsafe impl ::windows::core::Interface for IWMClientConnections {
    type Vtable = IWMClientConnectionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnectionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMClientConnections2(::windows::core::IUnknown);
impl IWMClientConnections2 {
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: WM_CLIENT_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(pwsznetworkaddress), ::core::mem::transmute(pcchnetworkaddress), ::core::mem::transmute(pwszport), ::core::mem::transmute(pcchport), ::core::mem::transmute(pwszdnsname), ::core::mem::transmute(pcchdnsname)).ok()
    }
}
impl ::core::convert::From<IWMClientConnections2> for IWMClientConnections {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for IWMClientConnections {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMClientConnections> for IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMClientConnections> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMClientConnections> for &IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMClientConnections> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMClientConnections2> for ::windows::core::IUnknown {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for ::windows::core::IUnknown {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections2 {}
unsafe impl ::windows::core::Interface for IWMClientConnections2 {
    type Vtable = IWMClientConnections2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMCodecAMVideoAccelerator(::windows::core::IUnknown);
impl IWMCodecAMVideoAccelerator {
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn SetAcceleratorInterface<'a, Param0: ::windows::core::IntoParam<'a, super::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piamva.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub unsafe fn NegotiateConnection(&self, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmediatype)).ok()
    }
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::core::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMCodecAMVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMCodecAMVideoAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecAMVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecAMVideoAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecAMVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCodecAMVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecAMVideoAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecAMVideoAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecAMVideoAccelerator {}
unsafe impl ::windows::core::Interface for IWMCodecAMVideoAccelerator {
    type Vtable = IWMCodecAMVideoAcceleratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd98ee251_34e0_4a2d_9312_9b4c788d9fa1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecAMVideoAcceleratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Media_DirectShow")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMCodecInfo(::windows::core::IUnknown);
impl IWMCodecInfo {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
}
impl ::core::convert::From<IWMCodecInfo> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo {}
unsafe impl ::windows::core::Interface for IWMCodecInfo {
    type Vtable = IWMCodecInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMCodecInfo2(::windows::core::IUnknown);
impl IWMCodecInfo2 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for &IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo2> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo2 {}
unsafe impl ::windows::core::Interface for IWMCodecInfo2 {
    type Vtable = IWMCodecInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMCodecInfo3(::windows::core::IUnknown);
impl IWMCodecInfo3 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecFormatProp<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: Param3, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecProp<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCodecEnumerationSetting<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecEnumerationSetting<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo2> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo2> for &IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for &IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo3 {}
unsafe impl ::windows::core::Interface for IWMCodecInfo3 {
    type Vtable = IWMCodecInfo3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMCodecVideoAccelerator(::windows::core::IUnknown);
impl IWMCodecVideoAccelerator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub unsafe fn NegotiateConnection<'a, Param0: ::windows::core::IntoParam<'a, super::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piamva.into_param().abi(), ::core::mem::transmute(pmediatype)).ok()
    }
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::core::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMCodecVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMCodecVideoAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecVideoAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCodecVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecVideoAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecVideoAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecVideoAccelerator {}
unsafe impl ::windows::core::Interface for IWMCodecVideoAccelerator {
    type Vtable = IWMCodecVideoAcceleratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x990641b0_739f_4e94_a808_9888da8f75af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecVideoAcceleratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMCredentialCallback(::windows::core::IUnknown);
impl IWMCredentialCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszrealm: Param0, pwszsite: Param1, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszrealm.into_param().abi(), pwszsite.into_param().abi(), ::core::mem::transmute(pwszuser), ::core::mem::transmute(cchuser), ::core::mem::transmute(pwszpassword), ::core::mem::transmute(cchpassword), ::core::mem::transmute(hrstatus), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IWMCredentialCallback> for ::windows::core::IUnknown {
    fn from(value: IWMCredentialCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCredentialCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMCredentialCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCredentialCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMCredentialCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCredentialCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCredentialCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCredentialCallback {}
unsafe impl ::windows::core::Interface for IWMCredentialCallback {
    type Vtable = IWMCredentialCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMEditor(::windows::core::IUnknown);
impl IWMDRMEditor {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMEditor> for ::windows::core::IUnknown {
    fn from(value: IWMDRMEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMEditor> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMEditor {}
unsafe impl ::windows::core::Interface for IWMDRMEditor {
    type Vtable = IWMDRMEditorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMMessageParser(::windows::core::IUnknown);
impl IWMDRMMessageParser {
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbregistrationreqmsg), ::core::mem::transmute(cbregistrationreqmsg), ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber), ::core::mem::transmute(pbstraction)).ok()
    }
}
impl ::core::convert::From<IWMDRMMessageParser> for ::windows::core::IUnknown {
    fn from(value: IWMDRMMessageParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMMessageParser> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMMessageParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMMessageParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMMessageParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMMessageParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMMessageParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMMessageParser {}
unsafe impl ::windows::core::Interface for IWMDRMMessageParser {
    type Vtable = IWMDRMMessageParserVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParserVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMReader(::windows::core::IUnknown);
impl IWMDRMReader {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader {}
unsafe impl ::windows::core::Interface for IWMDRMReader {
    type Vtable = IWMDRMReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMReader2(::windows::core::IUnknown);
impl IWMDRMReader2 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader2> for IWMDRMReader {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for IWMDRMReader {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for &IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader2 {}
unsafe impl ::windows::core::Interface for IWMDRMReader2 {
    type Vtable = IWMDRMReader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMDRMReader3(::windows::core::IUnknown);
impl IWMDRMReader3 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppguids), ::core::mem::transmute(pcguids)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader2> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader2> for &IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for &IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader3 {}
unsafe impl ::windows::core::Interface for IWMDRMReader3 {
    type Vtable = IWMDRMReader3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMDRMTranscryptionManager(::windows::core::IUnknown);
impl IWMDRMTranscryptionManager {
    pub unsafe fn CreateTranscryptor(&self) -> ::windows::core::Result<IWMDRMTranscryptor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDRMTranscryptor>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptionManager> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptionManager> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptionManager {}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptionManager {
    type Vtable = IWMDRMTranscryptionManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManagerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWMDRMTranscryptor(::windows::core::IUnknown);
impl IWMDRMTranscryptor {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hnstime)).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMTranscryptor> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMTranscryptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor {}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor {
    type Vtable = IWMDRMTranscryptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMDRMTranscryptor2(::windows::core::IUnknown);
impl IWMDRMTranscryptor2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hnstime)).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SeekEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration), ::core::mem::transmute(flrate), fincludefileheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZeroAdjustTimestamps<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetSeekStartTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMTranscryptor> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMTranscryptor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMTranscryptor> for &IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMTranscryptor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor2 {}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor2 {
    type Vtable = IWMDRMTranscryptor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMDRMWriter(::windows::core::IUnknown);
impl IWMDRMWriter {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter {}
unsafe impl ::windows::core::Interface for IWMDRMWriter {
    type Vtable = IWMDRMWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMWriter2(::windows::core::IUnknown);
impl IWMDRMWriter2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for &IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter2 {}
unsafe impl ::windows::core::Interface for IWMDRMWriter2 {
    type Vtable = IWMDRMWriter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMDRMWriter3(::windows::core::IUnknown);
impl IWMDRMWriter3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pimportinitstruct)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter2> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter2> for &IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for &IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter3 {}
unsafe impl ::windows::core::Interface for IWMDRMWriter3 {
    type Vtable = IWMDRMWriter3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMDeviceRegistration(::windows::core::IUnknown);
impl IWMDeviceRegistration {
    pub unsafe fn RegisterDevice<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(pbcertificate), ::core::mem::transmute(cbcertificate), serialnumber.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn UnregisterDevice<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(pbcertificate), ::core::mem::transmute(cbcertificate), serialnumber.into_param().abi()).ok()
    }
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn GetNextRegisteredDevice(&self) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    pub unsafe fn GetRegisteredDeviceByID<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: Param3) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(pbcertificate), ::core::mem::transmute(cbcertificate), serialnumber.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
}
impl ::core::convert::From<IWMDeviceRegistration> for ::windows::core::IUnknown {
    fn from(value: IWMDeviceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceRegistration> for ::windows::core::IUnknown {
    fn from(value: &IWMDeviceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDeviceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMDeviceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDeviceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceRegistration {}
unsafe impl ::windows::core::Interface for IWMDeviceRegistration {
    type Vtable = IWMDeviceRegistrationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistrationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMGetSecureChannel(::windows::core::IUnknown);
impl IWMGetSecureChannel {
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> ::windows::core::Result<IWMSecureChannel> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMSecureChannel>(result__)
    }
}
impl ::core::convert::From<IWMGetSecureChannel> for ::windows::core::IUnknown {
    fn from(value: IWMGetSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMGetSecureChannel> for ::windows::core::IUnknown {
    fn from(value: &IWMGetSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMGetSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMGetSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMGetSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMGetSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMGetSecureChannel {}
unsafe impl ::windows::core::Interface for IWMGetSecureChannel {
    type Vtable = IWMGetSecureChannelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc0598_c3d2_11d3_bedf_00c04f612986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMGetSecureChannelVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWMHeaderInfo(::windows::core::IUnknown);
impl IWMHeaderInfo {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMHeaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo {}
unsafe impl ::windows::core::Interface for IWMHeaderInfo {
    type Vtable = IWMHeaderInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMHeaderInfo2(::windows::core::IUnknown);
impl IWMHeaderInfo2 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for &IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo2 {}
unsafe impl ::windows::core::Interface for IWMHeaderInfo2 {
    type Vtable = IWMHeaderInfo2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMHeaderInfo3(::windows::core::IUnknown);
impl IWMHeaderInfo3 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeIndices<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pwszname: Param1, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pwszname.into_param().abi(), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pwindices), ::core::mem::transmute(pwcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwdatalength)).ok()
    }
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAttribute<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wstreamnum: u16, pszname: Param1, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(pwindex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwlength)).ok()
    }
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddCodecInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0, pwszdescription: Param1, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszdescription.into_param().abi(), ::core::mem::transmute(codectype), ::core::mem::transmute(cbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo2> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo2> for &IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for &IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo3 {}
unsafe impl ::windows::core::Interface for IWMHeaderInfo3 {
    type Vtable = IWMHeaderInfo3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMIStreamProps(::windows::core::IUnknown);
impl IWMIStreamProps {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMIStreamProps> for ::windows::core::IUnknown {
    fn from(value: IWMIStreamProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIStreamProps> for ::windows::core::IUnknown {
    fn from(value: &IWMIStreamProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIStreamProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMIStreamProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIStreamProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIStreamProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIStreamProps {}
unsafe impl ::windows::core::Interface for IWMIStreamProps {
    type Vtable = IWMIStreamPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMImageInfo(::windows::core::IUnknown);
impl IWMImageInfo {
    pub unsafe fn GetImageCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchmimetype), ::core::mem::transmute(pwszmimetype), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pimagetype), ::core::mem::transmute(pcbimagedata), ::core::mem::transmute(pbimagedata)).ok()
    }
}
impl ::core::convert::From<IWMImageInfo> for ::windows::core::IUnknown {
    fn from(value: IWMImageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMImageInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMImageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMImageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMImageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMImageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMImageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMImageInfo {}
unsafe impl ::windows::core::Interface for IWMImageInfo {
    type Vtable = IWMImageInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMIndexer(::windows::core::IUnknown);
impl IWMIndexer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMIndexer> for ::windows::core::IUnknown {
    fn from(value: IWMIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer> for ::windows::core::IUnknown {
    fn from(value: &IWMIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer {}
unsafe impl ::windows::core::Interface for IWMIndexer {
    type Vtable = IWMIndexerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMIndexer2(::windows::core::IUnknown);
impl IWMIndexer2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(nindexertype), ::core::mem::transmute(pvinterval), ::core::mem::transmute(pvindextype)).ok()
    }
}
impl ::core::convert::From<IWMIndexer2> for IWMIndexer {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for IWMIndexer {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMIndexer> for IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMIndexer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMIndexer> for &IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMIndexer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMIndexer2> for ::windows::core::IUnknown {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for ::windows::core::IUnknown {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer2 {}
unsafe impl ::windows::core::Interface for IWMIndexer2 {
    type Vtable = IWMIndexer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMInputMediaProps(::windows::core::IUnknown);
impl IWMInputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGroupName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMInputMediaProps> for IWMMediaProps {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for IWMMediaProps {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMInputMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMInputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMInputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInputMediaProps {}
unsafe impl ::windows::core::Interface for IWMInputMediaProps {
    type Vtable = IWMInputMediaPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMLanguageList(::windows::core::IUnknown);
impl IWMLanguageList {
    pub unsafe fn GetLanguageCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLanguageByRFC1766String<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszlanguagestring.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
}
impl ::core::convert::From<IWMLanguageList> for ::windows::core::IUnknown {
    fn from(value: IWMLanguageList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLanguageList> for ::windows::core::IUnknown {
    fn from(value: &IWMLanguageList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLanguageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMLanguageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLanguageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLanguageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLanguageList {}
unsafe impl ::windows::core::Interface for IWMLanguageList {
    type Vtable = IWMLanguageListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMLicenseBackup(::windows::core::IUnknown);
impl IWMLicenseBackup {
    pub unsafe fn BackupLicenses<'a, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseBackup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseBackup> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseBackup> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMLicenseBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseBackup {}
unsafe impl ::windows::core::Interface for IWMLicenseBackup {
    type Vtable = IWMLicenseBackupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMLicenseRestore(::windows::core::IUnknown);
impl IWMLicenseRestore {
    pub unsafe fn RestoreLicenses<'a, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    pub unsafe fn CancelLicenseRestore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRestore> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseRestore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRestore> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseRestore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMLicenseRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRestore {}
unsafe impl ::windows::core::Interface for IWMLicenseRestore {
    type Vtable = IWMLicenseRestoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMLicenseRevocationAgent(::windows::core::IUnknown);
impl IWMLicenseRevocationAgent {
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmachineid), ::core::mem::transmute(dwmachineidlength), ::core::mem::transmute(pchallenge), ::core::mem::transmute(dwchallengelength), ::core::mem::transmute(pchallengeoutput), ::core::mem::transmute(pdwchallengeoutputlength)).ok()
    }
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(psignedlrb), ::core::mem::transmute(dwsignedlrblength), ::core::mem::transmute(psignedack), ::core::mem::transmute(pdwsignedacklength)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRevocationAgent> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseRevocationAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRevocationAgent> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseRevocationAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRevocationAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRevocationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRevocationAgent {}
unsafe impl ::windows::core::Interface for IWMLicenseRevocationAgent {
    type Vtable = IWMLicenseRevocationAgentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMMediaProps(::windows::core::IUnknown);
impl IWMMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
}
impl ::core::convert::From<IWMMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMediaProps {}
unsafe impl ::windows::core::Interface for IWMMediaProps {
    type Vtable = IWMMediaPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMMetadataEditor(::windows::core::IUnknown);
impl IWMMetadataEditor {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor> for ::windows::core::IUnknown {
    fn from(value: IWMMetadataEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor> for ::windows::core::IUnknown {
    fn from(value: &IWMMetadataEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMetadataEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMMetadataEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor {}
unsafe impl ::windows::core::Interface for IWMMetadataEditor {
    type Vtable = IWMMetadataEditorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMMetadataEditor2(::windows::core::IUnknown);
impl IWMMetadataEditor2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMetadataEditor> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMetadataEditor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMetadataEditor> for &IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMetadataEditor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for ::windows::core::IUnknown {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for ::windows::core::IUnknown {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor2 {}
unsafe impl ::windows::core::Interface for IWMMetadataEditor2 {
    type Vtable = IWMMetadataEditor2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMMutualExclusion(::windows::core::IUnknown);
impl IWMMutualExclusion {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion> for IWMStreamList {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for IWMStreamList {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion> for ::windows::core::IUnknown {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for ::windows::core::IUnknown {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion {}
unsafe impl ::windows::core::Interface for IWMMutualExclusion {
    type Vtable = IWMMutualExclusionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMMutualExclusion2(::windows::core::IUnknown);
impl IWMMutualExclusion2 {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn AddRecord(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwszrecordname), ::core::mem::transmute(pcchrecordname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRecordName<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wrecordnumber: u16, pwszrecordname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), pwszrecordname.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMutualExclusion> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMutualExclusion> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMutualExclusion> for &IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMutualExclusion> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMStreamList {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMStreamList {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for ::windows::core::IUnknown {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for ::windows::core::IUnknown {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion2 {}
unsafe impl ::windows::core::Interface for IWMMutualExclusion2 {
    type Vtable = IWMMutualExclusion2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMOutputMediaProps(::windows::core::IUnknown);
impl IWMOutputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamGroupName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMOutputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMOutputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMOutputMediaProps {}
unsafe impl ::windows::core::Interface for IWMOutputMediaProps {
    type Vtable = IWMOutputMediaPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMPacketSize(::windows::core::IUnknown);
impl IWMPacketSize {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize> for ::windows::core::IUnknown {
    fn from(value: IWMPacketSize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize> for ::windows::core::IUnknown {
    fn from(value: &IWMPacketSize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPacketSize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMPacketSize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize {}
unsafe impl ::windows::core::Interface for IWMPacketSize {
    type Vtable = IWMPacketSizeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSizeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMPacketSize2(::windows::core::IUnknown);
impl IWMPacketSize2 {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
    pub unsafe fn GetMinPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwminpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize2> for IWMPacketSize {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for IWMPacketSize {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMPacketSize> for IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMPacketSize> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMPacketSize> for &IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMPacketSize> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMPacketSize2> for ::windows::core::IUnknown {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for ::windows::core::IUnknown {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize2 {}
unsafe impl ::windows::core::Interface for IWMPacketSize2 {
    type Vtable = IWMPacketSize2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMPlayerHook(::windows::core::IUnknown);
impl IWMPlayerHook {
    pub unsafe fn PreDecode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMPlayerHook> for ::windows::core::IUnknown {
    fn from(value: IWMPlayerHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerHook> for ::windows::core::IUnknown {
    fn from(value: &IWMPlayerHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPlayerHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMPlayerHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerHook {}
unsafe impl ::windows::core::Interface for IWMPlayerHook {
    type Vtable = IWMPlayerHookVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHookVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWMPlayerTimestampHook(::windows::core::IUnknown);
impl IWMPlayerTimestampHook {
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtin), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IWMPlayerTimestampHook> for ::windows::core::IUnknown {
    fn from(value: IWMPlayerTimestampHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerTimestampHook> for ::windows::core::IUnknown {
    fn from(value: &IWMPlayerTimestampHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerTimestampHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerTimestampHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerTimestampHook {}
unsafe impl ::windows::core::Interface for IWMPlayerTimestampHook {
    type Vtable = IWMPlayerTimestampHookVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28580dda_d98e_48d0_b7ae_69e473a02825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerTimestampHookVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWMProfile(::windows::core::IUnknown);
impl IWMProfile {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
}
impl ::core::convert::From<IWMProfile> for ::windows::core::IUnknown {
    fn from(value: IWMProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile {}
unsafe impl ::windows::core::Interface for IWMProfile {
    type Vtable = IWMProfileVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProfile2(::windows::core::IUnknown);
impl IWMProfile2 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IWMProfile2> for IWMProfile {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for IWMProfile {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for &IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile2> for ::windows::core::IUnknown {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile2 {}
unsafe impl ::windows::core::Interface for IWMProfile2 {
    type Vtable = IWMProfile2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProfile3(::windows::core::IUnknown);
impl IWMProfile3 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetStorageFormat(&self) -> ::windows::core::Result<WMT_STORAGE_FORMAT> {
        let mut result__: WMT_STORAGE_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STORAGE_FORMAT>(result__)
    }
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(nstorageformat)).ok()
    }
    pub unsafe fn GetBandwidthSharingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbsindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMBandwidthSharing>(result__)
    }
    pub unsafe fn RemoveBandwidthSharing<'a, Param0: ::windows::core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn AddBandwidthSharing<'a, Param0: ::windows::core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    pub unsafe fn CreateNewBandwidthSharing(&self) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMBandwidthSharing>(result__)
    }
    pub unsafe fn GetStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamPrioritization>(result__)
    }
    pub unsafe fn SetStreamPrioritization<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamPrioritization>>(&self, psp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), psp.into_param().abi()).ok()
    }
    pub unsafe fn RemoveStreamPrioritization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CreateNewStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamPrioritization>(result__)
    }
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(msduration), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile2 {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile2 {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile2> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile2> for &IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for &IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for ::windows::core::IUnknown {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile3 {}
unsafe impl ::windows::core::Interface for IWMProfile3 {
    type Vtable = IWMProfile3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProfileManager(::windows::core::IUnknown);
impl IWMProfileManager {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprofile: Param0) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
}
impl ::core::convert::From<IWMProfileManager> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager {}
unsafe impl ::windows::core::Interface for IWMProfileManager {
    type Vtable = IWMProfileManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProfileManager2(::windows::core::IUnknown);
impl IWMProfileManager2 {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprofile: Param0) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwversion)).ok()
    }
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion)).ok()
    }
}
impl ::core::convert::From<IWMProfileManager2> for IWMProfileManager {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for IWMProfileManager {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfileManager> for IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfileManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfileManager> for &IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfileManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfileManager2> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager2 {}
unsafe impl ::windows::core::Interface for IWMProfileManager2 {
    type Vtable = IWMProfileManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProfileManagerLanguage(::windows::core::IUnknown);
impl IWMProfileManagerLanguage {
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wlangid)).ok()
    }
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wlangid)).ok()
    }
}
impl ::core::convert::From<IWMProfileManagerLanguage> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManagerLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManagerLanguage> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManagerLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManagerLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManagerLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManagerLanguage {}
unsafe impl ::windows::core::Interface for IWMProfileManagerLanguage {
    type Vtable = IWMProfileManagerLanguageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMPropertyVault(::windows::core::IUnknown);
impl IWMPropertyVault {
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszname: Param0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pszname), ::core::mem::transmute(pdwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    pub unsafe fn CopyPropertiesFrom<'a, Param0: ::windows::core::IntoParam<'a, IWMPropertyVault>>(&self, piwmpropertyvault: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), piwmpropertyvault.into_param().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMPropertyVault> for ::windows::core::IUnknown {
    fn from(value: IWMPropertyVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPropertyVault> for ::windows::core::IUnknown {
    fn from(value: &IWMPropertyVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPropertyVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMPropertyVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPropertyVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPropertyVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPropertyVault {}
unsafe impl ::windows::core::Interface for IWMPropertyVault {
    type Vtable = IWMPropertyVaultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVaultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMProximityDetection(::windows::core::IUnknown);
impl IWMProximityDetection {
    pub unsafe fn StartDetection<'a, Param6: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param6, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbregistrationmsg), ::core::mem::transmute(cbregistrationmsg), ::core::mem::transmute(pblocaladdress), ::core::mem::transmute(cblocaladdress), ::core::mem::transmute(dwextraportsallowed), ::core::mem::transmute(ppregistrationresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMProximityDetection> for ::windows::core::IUnknown {
    fn from(value: IWMProximityDetection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProximityDetection> for ::windows::core::IUnknown {
    fn from(value: &IWMProximityDetection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProximityDetection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMProximityDetection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProximityDetection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProximityDetection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProximityDetection {}
unsafe impl ::windows::core::Interface for IWMProximityDetection {
    type Vtable = IWMProximityDetectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReader(::windows::core::IUnknown);
impl IWMReader {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMReader> for ::windows::core::IUnknown {
    fn from(value: IWMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReader> for ::windows::core::IUnknown {
    fn from(value: &IWMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReader {}
unsafe impl ::windows::core::Interface for IWMReader {
    type Vtable = IWMReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderAccelerator(::windows::core::IUnknown);
impl IWMReaderAccelerator {
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(riid), ::core::mem::transmute(ppvcodecinterface)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(psubtype)).ok()
    }
}
impl ::core::convert::From<IWMReaderAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAccelerator {}
unsafe impl ::windows::core::Interface for IWMReaderAccelerator {
    type Vtable = IWMReaderAcceleratorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAcceleratorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced(::windows::core::IUnknown);
impl IWMReaderAdvanced {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced {
    type Vtable = IWMReaderAdvancedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvancedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced2(::windows::core::IUnknown);
impl IWMReaderAdvanced2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced2 {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced2 {
    type Vtable = IWMReaderAdvanced2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced3(::windows::core::IUnknown);
impl IWMReaderAdvanced3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced3 {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced3 {
    type Vtable = IWMReaderAdvanced3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced4(::windows::core::IUnknown);
impl IWMReaderAdvanced4 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced4 {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced4 {
    type Vtable = IWMReaderAdvanced4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced5(::windows::core::IUnknown);
impl IWMReaderAdvanced5 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced5 {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced5 {
    type Vtable = IWMReaderAdvanced5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderAdvanced6(::windows::core::IUnknown);
impl IWMReaderAdvanced6 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).49)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).50)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbcertificate), ::core::mem::transmute(cbcertificate), ::core::mem::transmute(dwcertificatetype), ::core::mem::transmute(dwflags), ::core::mem::transmute(pbinitializationvector), ::core::mem::transmute(pcbinitializationvector)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced5> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced5> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced6 {}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced6 {
    type Vtable = IWMReaderAdvanced6Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderAllocatorEx(::windows::core::IUnknown);
impl IWMReaderAllocatorEx {
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAllocatorEx> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAllocatorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAllocatorEx> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAllocatorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAllocatorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAllocatorEx {}
unsafe impl ::windows::core::Interface for IWMReaderAllocatorEx {
    type Vtable = IWMReaderAllocatorExVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorExVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderCallback(::windows::core::IUnknown);
impl IWMReaderCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallback> for IWMStatusCallback {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for IWMStatusCallback {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for &IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderCallback> for ::windows::core::IUnknown {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallback {}
unsafe impl ::windows::core::Interface for IWMReaderCallback {
    type Vtable = IWMReaderCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderCallbackAdvanced(::windows::core::IUnknown);
impl IWMReaderCallbackAdvanced {
    pub unsafe fn OnStreamSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnscurrenttime), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamcount), ::core::mem::transmute(pstreamnumbers), ::core::mem::transmute(pselections), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(pmediatype), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallbackAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMReaderCallbackAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallbackAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderCallbackAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallbackAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallbackAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallbackAdvanced {}
unsafe impl ::windows::core::Interface for IWMReaderCallbackAdvanced {
    type Vtable = IWMReaderCallbackAdvancedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvancedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderNetworkConfig(::windows::core::IUnknown);
impl IWMReaderNetworkConfig {
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(cranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<WMT_PROXY_SETTINGS> {
        let mut result__: WMT_PROXY_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszurl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig> for ::windows::core::IUnknown {
    fn from(value: IWMReaderNetworkConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig {}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig {
    type Vtable = IWMReaderNetworkConfigVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfigVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderNetworkConfig2(::windows::core::IUnknown);
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(cranges)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<WMT_PROXY_SETTINGS> {
        let mut result__: WMT_PROXY_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszurl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableContentCaching(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableContentCaching<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablecontentcaching: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).37)(::core::mem::transmute_copy(self), fenablecontentcaching.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableFastCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablefastcache: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).39)(::core::mem::transmute_copy(self), fenablefastcache.into_param().abi()).ok()
    }
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).40)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).41)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsaccelduration)).ok()
    }
    pub unsafe fn GetAutoReconnectLimit(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).42)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).43)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwautoreconnectlimit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableResends(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).44)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableResends<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableresends: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).45)(::core::mem::transmute_copy(self), fenableresends.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableThinning(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).46)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableThinning<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablethinning: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).47)(::core::mem::transmute_copy(self), fenablethinning.into_param().abi()).ok()
    }
    pub unsafe fn GetMaxNetPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).48)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderNetworkConfig> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderNetworkConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderNetworkConfig> for &IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderNetworkConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for ::windows::core::IUnknown {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig2 {}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig2 {
    type Vtable = IWMReaderNetworkConfig2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderPlaylistBurn(::windows::core::IUnknown);
impl IWMReaderPlaylistBurn {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitPlaylistBurn<'a, Param2: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: Param2, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(ppwszfilenames), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrburnresult)).ok()
    }
}
impl ::core::convert::From<IWMReaderPlaylistBurn> for ::windows::core::IUnknown {
    fn from(value: IWMReaderPlaylistBurn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderPlaylistBurn> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderPlaylistBurn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderPlaylistBurn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderPlaylistBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderPlaylistBurn {}
unsafe impl ::windows::core::Interface for IWMReaderPlaylistBurn {
    type Vtable = IWMReaderPlaylistBurnVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurnVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderStreamClock(::windows::core::IUnknown);
impl IWMReaderStreamClock {
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcnsnow)).ok()
    }
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnswhen), ::core::mem::transmute(pvparam), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimerid)).ok()
    }
}
impl ::core::convert::From<IWMReaderStreamClock> for ::windows::core::IUnknown {
    fn from(value: IWMReaderStreamClock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderStreamClock> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderStreamClock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderStreamClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderStreamClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderStreamClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderStreamClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderStreamClock {}
unsafe impl ::windows::core::Interface for IWMReaderStreamClock {
    type Vtable = IWMReaderStreamClockVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClockVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderTimecode(::windows::core::IUnknown);
impl IWMReaderTimecode {
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(wrangenum), ::core::mem::transmute(pstarttimecode), ::core::mem::transmute(pendtimecode)).ok()
    }
}
impl ::core::convert::From<IWMReaderTimecode> for ::windows::core::IUnknown {
    fn from(value: IWMReaderTimecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTimecode> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderTimecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderTimecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderTimecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTimecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTimecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTimecode {}
unsafe impl ::windows::core::Interface for IWMReaderTimecode {
    type Vtable = IWMReaderTimecodeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecodeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMReaderTypeNegotiation(::windows::core::IUnknown);
impl IWMReaderTypeNegotiation {
    pub unsafe fn TryOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderTypeNegotiation> for ::windows::core::IUnknown {
    fn from(value: IWMReaderTypeNegotiation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTypeNegotiation> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderTypeNegotiation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTypeNegotiation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTypeNegotiation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTypeNegotiation {}
unsafe impl ::windows::core::Interface for IWMReaderTypeNegotiation {
    type Vtable = IWMReaderTypeNegotiationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiationVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IWMRegisterCallback(::windows::core::IUnknown);
impl IWMRegisterCallback {
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn Unadvise<'a, Param0: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMRegisterCallback> for ::windows::core::IUnknown {
    fn from(value: IWMRegisterCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisterCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMRegisterCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMRegisterCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMRegisterCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisterCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisterCallback {}
unsafe impl ::windows::core::Interface for IWMRegisterCallback {
    type Vtable = IWMRegisterCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMRegisteredDevice(::windows::core::IUnknown);
impl IWMRegisteredDevice {
    pub unsafe fn GetDeviceSerialNumber(&self) -> ::windows::core::Result<DRM_VAL16> {
        let mut result__: DRM_VAL16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DRM_VAL16>(result__)
    }
    pub unsafe fn GetDeviceCertificate(&self) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn GetDeviceType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Approve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fapprove: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), fapprove.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsApproved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWmdrmCompliant(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOpened(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMRegisteredDevice> for ::windows::core::IUnknown {
    fn from(value: IWMRegisteredDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisteredDevice> for ::windows::core::IUnknown {
    fn from(value: &IWMRegisteredDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMRegisteredDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMRegisteredDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisteredDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisteredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisteredDevice {}
unsafe impl ::windows::core::Interface for IWMRegisteredDevice {
    type Vtable = IWMRegisteredDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMSBufferAllocator(::windows::core::IUnknown);
impl IWMSBufferAllocator {
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
}
impl ::core::convert::From<IWMSBufferAllocator> for ::windows::core::IUnknown {
    fn from(value: IWMSBufferAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSBufferAllocator> for ::windows::core::IUnknown {
    fn from(value: &IWMSBufferAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSBufferAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSBufferAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSBufferAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSBufferAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSBufferAllocator {}
unsafe impl ::windows::core::Interface for IWMSBufferAllocator {
    type Vtable = IWMSBufferAllocatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, INSNetSourceCreator>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, psharednamespace: Param0, pnamespacenode: Param1, pnetsourcecreator: Param2, fembeddedinserver: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psharednamespace.into_param().abi(), pnamespacenode.into_param().abi(), pnetsourcecreator.into_param().abi(), fembeddedinserver.into_param().abi()).ok()
    }
    pub unsafe fn GetNetSourceCreator(&self) -> ::windows::core::Result<INSNetSourceCreator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INSNetSourceCreator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrname: Param1, bstrpassword: Param2, fpersist: Param3, fconfirmedgood: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi()).ok()
    }
    pub unsafe fn GetCredentialFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    pub unsafe fn RegisterProxyFailure(&self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(dwproxycontext)).ok()
    }
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproxycontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproxycontext), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource {}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource {
    type Vtable = IWMSInternalAdminNetSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb23e5f_d127_4afb_8d02_ae5b66d54c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource2(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource2> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource2> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource2 {}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource2 {
    type Vtable = IWMSInternalAdminNetSource2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe74d58c3_cf77_4b51_af17_744687c43eae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource3(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    pub unsafe fn GetNetSourceCreator2(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pqwproxycontext)).ok()
    }
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(qwproxycontext)).ok()
    }
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(qwproxycontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(qwproxycontext), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        bstrname: Param3,
        bstrpassword: Param4,
        fpersist: Param5,
        fconfirmedgood: Param6,
        fcleartextauthentication: Param7,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi(), fcleartextauthentication.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, fcleartextauthentication: Param3, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), fcleartextauthentication.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSInternalAdminNetSource2> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSInternalAdminNetSource2> for &IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource3 {}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource3 {
    type Vtable = IWMSInternalAdminNetSource3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b63d08e_4590_44af_9eb3_57ff1e73bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMSecureChannel(::windows::core::IUnknown);
impl IWMSecureChannel {
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn WMSC_AddCertificate<'a, Param0: ::windows::core::IntoParam<'a, IWMAuthorizer>>(&self, pcert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcert.into_param().abi()).ok()
    }
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbcertsig), ::core::mem::transmute(cbcertsig)).ok()
    }
    pub unsafe fn WMSC_Connect<'a, Param0: ::windows::core::IntoParam<'a, IWMSecureChannel>>(&self, potherside: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), potherside.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WMSC_IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn WMSC_Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbcertificate), ::core::mem::transmute(pdwsignature)).ok()
    }
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    pub unsafe fn WMSC_Lock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WMSC_Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata)).ok()
    }
}
impl ::core::convert::From<IWMSecureChannel> for IWMAuthorizer {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for IWMAuthorizer {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAuthorizer> for IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAuthorizer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAuthorizer> for &IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAuthorizer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSecureChannel> for ::windows::core::IUnknown {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for ::windows::core::IUnknown {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSecureChannel {}
unsafe impl ::windows::core::Interface for IWMSecureChannel {
    type Vtable = IWMSecureChannelVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2720598a_d0f2_4189_bd10_91c46ef0936f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSecureChannelVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMStatusCallback(::windows::core::IUnknown);
impl IWMStatusCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMStatusCallback> for ::windows::core::IUnknown {
    fn from(value: IWMStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStatusCallback {}
unsafe impl ::windows::core::Interface for IWMStatusCallback {
    type Vtable = IWMStatusCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMStreamConfig(::windows::core::IUnknown);
impl IWMStreamConfig {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStreamConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig {}
unsafe impl ::windows::core::Interface for IWMStreamConfig {
    type Vtable = IWMStreamConfigVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfigVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMStreamConfig2(::windows::core::IUnknown);
impl IWMStreamConfig2 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: WMT_TRANSPORT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(cbextensionsysteminfo)).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for &IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig2> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig2 {}
unsafe impl ::windows::core::Interface for IWMStreamConfig2 {
    type Vtable = IWMStreamConfig2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMStreamConfig3(::windows::core::IUnknown);
impl IWMStreamConfig3 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamName(&self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectionName(&self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: WMT_TRANSPORT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(cbextensionsysteminfo)).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pwszlanguagestring.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig2> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig2> for &IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for &IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig3 {}
unsafe impl ::windows::core::Interface for IWMStreamConfig3 {
    type Vtable = IWMStreamConfig3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMStreamList(::windows::core::IUnknown);
impl IWMStreamList {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
}
impl ::core::convert::From<IWMStreamList> for ::windows::core::IUnknown {
    fn from(value: IWMStreamList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamList> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStreamList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamList {}
unsafe impl ::windows::core::Interface for IWMStreamList {
    type Vtable = IWMStreamListVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamListVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMStreamPrioritization(::windows::core::IUnknown);
impl IWMStreamPrioritization {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(pcrecords)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(crecords)).ok()
    }
}
impl ::core::convert::From<IWMStreamPrioritization> for ::windows::core::IUnknown {
    fn from(value: IWMStreamPrioritization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamPrioritization> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamPrioritization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamPrioritization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMStreamPrioritization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamPrioritization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamPrioritization {}
unsafe impl ::windows::core::Interface for IWMStreamPrioritization {
    type Vtable = IWMStreamPrioritizationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritizationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMSyncReader(::windows::core::IUnknown);
impl IWMSyncReader {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMSyncReader> for ::windows::core::IUnknown {
    fn from(value: IWMSyncReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader> for ::windows::core::IUnknown {
    fn from(value: &IWMSyncReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSyncReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSyncReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader {}
unsafe impl ::windows::core::Interface for IWMSyncReader {
    type Vtable = IWMSyncReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
pub struct IWMSyncReader2(::windows::core::IUnknown);
impl IWMSyncReader2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pstart), ::core::mem::transmute(pend)).ok()
    }
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, dwoutputnum: u32, pallocator: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMReaderAllocatorEx>(result__)
    }
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, wstreamnum: u16, pallocator: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pallocator.into_param().abi()).ok()
    }
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMReaderAllocatorEx>(result__)
    }
}
impl ::core::convert::From<IWMSyncReader2> for IWMSyncReader {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for IWMSyncReader {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSyncReader> for IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSyncReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSyncReader> for &IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSyncReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSyncReader2> for ::windows::core::IUnknown {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for ::windows::core::IUnknown {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader2 {}
unsafe impl ::windows::core::Interface for IWMSyncReader2 {
    type Vtable = IWMSyncReader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMVideoMediaProps(::windows::core::IUnknown);
impl IWMVideoMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lltime)).ok()
    }
    pub unsafe fn GetQuality(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQuality(&self, dwquality: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwquality)).ok()
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMVideoMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMVideoMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoMediaProps {}
unsafe impl ::windows::core::Interface for IWMVideoMediaProps {
    type Vtable = IWMVideoMediaPropsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaPropsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWatermarkInfo(::windows::core::IUnknown);
impl IWMWatermarkInfo {
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows::core::Result<WMT_WATERMARK_ENTRY> {
        let mut result__: WMT_WATERMARK_ENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_WATERMARK_ENTRY>(result__)
    }
}
impl ::core::convert::From<IWMWatermarkInfo> for ::windows::core::IUnknown {
    fn from(value: IWMWatermarkInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWatermarkInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMWatermarkInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWatermarkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWatermarkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWatermarkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWatermarkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWatermarkInfo {}
unsafe impl ::windows::core::Interface for IWMWatermarkInfo {
    type Vtable = IWMWatermarkInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMWriter(::windows::core::IUnknown);
impl IWMWriter {
    pub unsafe fn SetProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile)).ok()
    }
    pub unsafe fn SetProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>>(&self, pprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprofile.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn GetInputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMInputMediaProps>(result__)
    }
    pub unsafe fn SetInputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMInputMediaProps>>(&self, dwinputnum: u32, pinput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pinput.into_param().abi()).ok()
    }
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMInputMediaProps>(result__)
    }
    pub unsafe fn BeginWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsamplesize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn WriteSample<'a, Param3: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriter> for ::windows::core::IUnknown {
    fn from(value: IWMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriter> for ::windows::core::IUnknown {
    fn from(value: &IWMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriter {}
unsafe impl ::windows::core::Interface for IWMWriter {
    type Vtable = IWMWriterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterAdvanced(::windows::core::IUnknown);
impl IWMWriterAdvanced {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMWriterAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced {}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced {
    type Vtable = IWMWriterAdvancedVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvancedVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterAdvanced2(::windows::core::IUnknown);
impl IWMWriterAdvanced2 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for &IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced2 {}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced2 {
    type Vtable = IWMWriterAdvanced2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMWriterAdvanced3(::windows::core::IUnknown);
impl IWMWriterAdvanced3 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(pvalue), ::core::mem::transmute(cblength)).ok()
    }
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS_EX> {
        let mut result__: WM_WRITER_STATISTICS_EX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS_EX>(result__)
    }
    pub unsafe fn SetNonBlocking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced2> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced2> for &IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for &IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced3 {}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced3 {
    type Vtable = IWMWriterAdvanced3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterFileSink(::windows::core::IUnknown);
impl IWMWriterFileSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink> for IWMWriterSink {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink {}
unsafe impl ::windows::core::Interface for IWMWriterFileSink {
    type Vtable = IWMWriterFileSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMWriterFileSink2(::windows::core::IUnknown);
impl IWMWriterFileSink2 {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for &IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink2 {}
unsafe impl ::windows::core::Interface for IWMWriterFileSink2 {
    type Vtable = IWMWriterFileSink2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMWriterFileSink3(::windows::core::IUnknown);
impl IWMWriterFileSink3 {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoIndexing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdoautoindexing: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), fdoautoindexing.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetControlStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), fshouldcontrolstartandstop.into_param().abi()).ok()
    }
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilesinkdataunit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnbufferedIO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, funbufferedio: Param0, frestrictmemusage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), funbufferedio.into_param().abi(), frestrictmemusage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnbufferedIO(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn CompleteOperations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink2> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink2> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink3 {}
unsafe impl ::windows::core::Interface for IWMWriterFileSink3 {
    type Vtable = IWMWriterFileSink3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterNetworkSink(::windows::core::IUnknown);
impl IWMWriterNetworkSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxclients)).ok()
    }
    pub unsafe fn GetMaximumClients(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    pub unsafe fn GetNetworkProtocol(&self) -> ::windows::core::Result<WMT_NET_PROTOCOL> {
        let mut result__: WMT_NET_PROTOCOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_NET_PROTOCOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHostURL(&self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwportnum)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterNetworkSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterNetworkSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterNetworkSink {}
unsafe impl ::windows::core::Interface for IWMWriterNetworkSink {
    type Vtable = IWMWriterNetworkSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterPostView(::windows::core::IUnknown);
impl IWMWriterPostView {
    pub unsafe fn SetPostViewCallback<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterPostViewCallback>>(&self, pcallback: Param0, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceivePostViewSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivepostviewsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivepostviewsamples.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMMediaProps>(result__)
    }
    pub unsafe fn SetPostViewProps<'a, Param1: ::windows::core::IntoParam<'a, IWMMediaProps>>(&self, wstreamnumber: u16, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), poutput.into_param().abi()).ok()
    }
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMMediaProps>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForPostView<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), fallocate.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterPostView> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPostView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostView> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPostView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPostView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterPostView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostView {}
unsafe impl ::windows::core::Interface for IWMWriterPostView {
    type Vtable = IWMWriterPostViewVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IWMWriterPostViewCallback(::windows::core::IUnknown);
impl IWMWriterPostViewCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn OnPostViewSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for &IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostViewCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostViewCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostViewCallback {}
unsafe impl ::windows::core::Interface for IWMWriterPostViewCallback {
    type Vtable = IWMWriterPostViewCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterPreprocess(::windows::core::IUnknown);
impl IWMWriterPreprocess {
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwnumpasses)).ok()
    }
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn PreprocessSample<'a, Param3: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IWMWriterPreprocess> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPreprocess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPreprocess> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPreprocess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPreprocess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterPreprocess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPreprocess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPreprocess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPreprocess {}
unsafe impl ::windows::core::Interface for IWMWriterPreprocess {
    type Vtable = IWMWriterPreprocessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocessVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterPushSink(::windows::core::IUnknown);
impl IWMWriterPushSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszurl: Param0, pwsztemplateurl: Param1, fautodestroy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pwsztemplateurl.into_param().abi(), fautodestroy.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EndSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterPushSink> for IWMWriterSink {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for IWMWriterSink {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPushSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPushSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPushSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPushSink {}
unsafe impl ::windows::core::Interface for IWMWriterPushSink {
    type Vtable = IWMWriterPushSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWMWriterSink(::windows::core::IUnknown);
impl IWMWriterSink {
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWMWriterSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterSink {}
unsafe impl ::windows::core::Interface for IWMWriterSink {
    type Vtable = IWMWriterSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
pub type NETSOURCE_URLCREDPOLICY_SETTINGS = i32;
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = 0i32;
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = 1i32;
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = 2i32;
pub type WEBSTREAM_SAMPLE_TYPE = i32;
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = 1i32;
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = 2i32;
#[inline]
pub unsafe fn WMCreateBackupRestorer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcallback: Param0) -> ::windows::core::Result<IWMLicenseBackup> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateBackupRestorer(pcallback: *mut ::core::ffi::c_void, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateBackupRestorer(pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMLicenseBackup>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateEditor() -> ::windows::core::Result<IWMMetadataEditor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateEditor(ppeditor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateEditor(::core::mem::transmute(&mut result__)).from_abi::<IWMMetadataEditor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateIndexer() -> ::windows::core::Result<IWMIndexer> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateIndexer(ppindexer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateIndexer(::core::mem::transmute(&mut result__)).from_abi::<IWMIndexer>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateProfileManager() -> ::windows::core::Result<IWMProfileManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateProfileManager(ppprofilemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateProfileManager(::core::mem::transmute(&mut result__)).from_abi::<IWMProfileManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateReader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::core::Result<IWMReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(&mut result__)).from_abi::<IWMReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateSyncReader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::core::Result<IWMSyncReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateSyncReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppsyncreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateSyncReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(&mut result__)).from_abi::<IWMSyncReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0) -> ::windows::core::Result<IWMWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriter(punkcert: *mut ::core::ffi::c_void, ppwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriter(punkcert.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> ::windows::core::Result<IWMWriterFileSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterFileSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterFileSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterFileSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> ::windows::core::Result<IWMWriterNetworkSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterNetworkSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterNetworkSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterNetworkSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> ::windows::core::Result<IWMWriterPushSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterPushSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterPushSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterPushSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl ::core::marker::Copy for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::clone::Clone for WMDRM_IMPORT_INIT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMDRM_IMPORT_INIT_STRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDRM_IMPORT_INIT_STRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WMIsContentProtected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszfilename: Param0, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMIsContentProtected(pwszfilename: super::super::Foundation::PWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        WMIsContentProtected(pwszfilename.into_param().abi(), ::core::mem::transmute(pfisprotected)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WMMEDIASUBTYPE_ACELPnet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMMPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMMPEG2VIDEOINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMMPEG2VIDEOINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows::core::GUID,
}
impl ::core::marker::Copy for WMSCRIPTFORMAT {}
impl ::core::clone::Clone for WMSCRIPTFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMSCRIPTFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMSCRIPTFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMSCRIPTFORMAT {}
impl ::core::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMSCRIPTTYPE_TwoStrings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
pub type WMT_ATTR_DATATYPE = i32;
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = 0i32;
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = 1i32;
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = 2i32;
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = 3i32;
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = 4i32;
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = 5i32;
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = 6i32;
pub type WMT_ATTR_IMAGETYPE = i32;
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = 1i32;
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = 2i32;
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = 3i32;
#[repr(C)]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: ::core::option::Option<INSSBuffer>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl ::core::clone::Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        Self { pBuffer: self.pBuffer.clone(), cbOffset: self.cbOffset, cbLength: self.cbLength }
    }
}
unsafe impl ::windows::core::Abi for WMT_BUFFER_SEGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::core::cmp::Eq for WMT_BUFFER_SEGMENT {}
impl ::core::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_CODEC_INFO_TYPE = i32;
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = 0i32;
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = 1i32;
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = -1i32;
#[repr(C)]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl ::core::marker::Copy for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_COLORSPACEINFO_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_CREDENTIAL_FLAGS = i32;
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = 1i32;
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = 2i32;
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = 4i32;
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = 8i32;
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = 16i32;
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
pub type WMT_DRMLA_TRUST = i32;
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = 0i32;
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = 1i32;
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = 2i32;
#[repr(C)]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl ::core::clone::Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        Self {
            packetHeaderBuffer: self.packetHeaderBuffer.clone(),
            cPayloads: self.cPayloads,
            pPayloadHeaderBuffers: self.pPayloadHeaderBuffers,
            cPayloadDataFragments: self.cPayloadDataFragments,
            pPayloadDataFragments: self.pPayloadDataFragments,
        }
    }
}
unsafe impl ::windows::core::Abi for WMT_FILESINK_DATA_UNIT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::core::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
impl ::core::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_FILESINK_MODE = i32;
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = 1i32;
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = 2i32;
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = 4i32;
pub type WMT_IMAGE_TYPE = i32;
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = 0i32;
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = 1i32;
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = 2i32;
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = 3i32;
pub type WMT_INDEXER_TYPE = i32;
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = 0i32;
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = 1i32;
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = 2i32;
pub type WMT_INDEX_TYPE = i32;
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = 1i32;
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = 2i32;
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = 3i32;
pub type WMT_MUSICSPEECH_CLASS_MODE = i32;
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = 0i32;
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = 1i32;
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = 2i32;
pub type WMT_NET_PROTOCOL = i32;
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = 0i32;
pub type WMT_OFFSET_FORMAT = i32;
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = 0i32;
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = 1i32;
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = 2i32;
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = 3i32;
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = 4i32;
#[repr(C)]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl ::core::clone::Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        Self { dwPayloadIndex: self.dwPayloadIndex, segmentData: self.segmentData.clone() }
    }
}
unsafe impl ::windows::core::Abi for WMT_PAYLOAD_FRAGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::core::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
impl ::core::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_PLAY_MODE = i32;
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = 0i32;
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = 1i32;
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = 2i32;
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = 3i32;
pub type WMT_PROXY_SETTINGS = i32;
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = 0i32;
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = 1i32;
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = 2i32;
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = 3i32;
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = 4i32;
pub type WMT_RIGHTS = i32;
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = 1i32;
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = 2i32;
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = 8i32;
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = 16i32;
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = 32i32;
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = 64i32;
pub const WMT_RIGHT_COPY: WMT_RIGHTS = 128i32;
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = 256i32;
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = 65536i32;
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = 131072i32;
pub type WMT_STATUS = i32;
pub const WMT_ERROR: WMT_STATUS = 0i32;
pub const WMT_OPENED: WMT_STATUS = 1i32;
pub const WMT_BUFFERING_START: WMT_STATUS = 2i32;
pub const WMT_BUFFERING_STOP: WMT_STATUS = 3i32;
pub const WMT_EOF: WMT_STATUS = 4i32;
pub const WMT_END_OF_FILE: WMT_STATUS = 4i32;
pub const WMT_END_OF_SEGMENT: WMT_STATUS = 5i32;
pub const WMT_END_OF_STREAMING: WMT_STATUS = 6i32;
pub const WMT_LOCATING: WMT_STATUS = 7i32;
pub const WMT_CONNECTING: WMT_STATUS = 8i32;
pub const WMT_NO_RIGHTS: WMT_STATUS = 9i32;
pub const WMT_MISSING_CODEC: WMT_STATUS = 10i32;
pub const WMT_STARTED: WMT_STATUS = 11i32;
pub const WMT_STOPPED: WMT_STATUS = 12i32;
pub const WMT_CLOSED: WMT_STATUS = 13i32;
pub const WMT_STRIDING: WMT_STATUS = 14i32;
pub const WMT_TIMER: WMT_STATUS = 15i32;
pub const WMT_INDEX_PROGRESS: WMT_STATUS = 16i32;
pub const WMT_SAVEAS_START: WMT_STATUS = 17i32;
pub const WMT_SAVEAS_STOP: WMT_STATUS = 18i32;
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = 19i32;
pub const WMT_NEW_METADATA: WMT_STATUS = 20i32;
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = 21i32;
pub const WMT_SOURCE_SWITCH: WMT_STATUS = 22i32;
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = 23i32;
pub const WMT_INDIVIDUALIZE: WMT_STATUS = 24i32;
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = 25i32;
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = 26i32;
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = 27i32;
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = 28i32;
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = 29i32;
pub const WMT_ERROR_WITHURL: WMT_STATUS = 30i32;
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = 31i32;
pub const WMT_CLIENT_CONNECT: WMT_STATUS = 32i32;
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = 33i32;
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = 34i32;
pub const WMT_RECONNECT_START: WMT_STATUS = 35i32;
pub const WMT_RECONNECT_END: WMT_STATUS = 36i32;
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = 37i32;
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = 38i32;
pub const WMT_SET_FEC_SPAN: WMT_STATUS = 39i32;
pub const WMT_PREROLL_READY: WMT_STATUS = 40i32;
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = 41i32;
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = 42i32;
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = 43i32;
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = 44i32;
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = 45i32;
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = 46i32;
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = 47i32;
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = 48i32;
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = 49i32;
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = 50i32;
pub const WMT_CONTENT_ENABLER: WMT_STATUS = 51i32;
pub type WMT_STORAGE_FORMAT = i32;
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = 0i32;
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = 1i32;
pub type WMT_STREAM_SELECTION = i32;
pub const WMT_OFF: WMT_STREAM_SELECTION = 0i32;
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = 1i32;
pub const WMT_ON: WMT_STREAM_SELECTION = 2i32;
#[repr(C, packed(2))]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl ::core::marker::Copy for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_TIMECODE_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_TIMECODE_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_TIMECODE_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_TIMECODE_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_TIMECODE_FRAMERATE = i32;
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = 0i32;
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = 1i32;
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = 2i32;
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = 3i32;
pub type WMT_TRANSPORT_TYPE = i32;
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = 0i32;
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = 1i32;
pub type WMT_VERSION = i32;
pub const WMT_VER_4_0: WMT_VERSION = 262144i32;
pub const WMT_VER_7_0: WMT_VERSION = 458752i32;
pub const WMT_VER_8_0: WMT_VERSION = 524288i32;
pub const WMT_VER_9_0: WMT_VERSION = 589824i32;
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[repr(C)]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_VIDEOIMAGE_SAMPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WMT_VIDEOIMAGE_SAMPLE2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows::core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WMT_WATERMARK_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WATERMARK_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WMT_WATERMARK_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WMT_WATERMARK_ENTRY_TYPE = i32;
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = 1i32;
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = 2i32;
#[repr(C)]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WMT_WEBSTREAM_FORMAT {}
impl ::core::clone::Clone for WMT_WEBSTREAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_WEBSTREAM_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
impl ::core::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl ::core::marker::Copy for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::clone::Clone for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_WEBSTREAM_SAMPLE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_SAMPLE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMVIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMVIDEOINFOHEADER2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WM_ADDRESS_ACCESSENTRY {}
impl ::core::clone::Clone for WM_ADDRESS_ACCESSENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_ADDRESS_ACCESSENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_ADDRESS_ACCESSENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
impl ::core::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WM_AETYPE = i32;
pub const WM_AETYPE_INCLUDE: WM_AETYPE = 105i32;
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = 101i32;
#[repr(C)]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_CLIENT_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: super::super::Foundation::PWSTR,
    pub pwszPort: super::super::Foundation::PWSTR,
    pub pwszDNSName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_CLIENT_PROPERTIES_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_CL_INTERLACED420: u32 = 0u32;
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
pub const WM_CT_INTERLACED: u32 = 128u32;
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
pub type WM_DM_INTERLACED_TYPE = i32;
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = 0i32;
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = 1i32;
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = 2i32;
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = 3i32;
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = 4i32;
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = 5i32;
pub type WM_DM_IT_FIRST_FRAME_COHERENCY = i32;
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = 0i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 1i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 2i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 3i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 4i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = 5i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 6i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 7i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 8i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 9i32;
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = 10i32;
#[repr(C, packed(1))]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for WM_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for WM_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_LEAKY_BUCKET_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_LEAKY_BUCKET_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_LEAKY_BUCKET_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_LEAKY_BUCKET_PAIR {}
impl ::core::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_MAX_STREAMS: u32 = 63u32;
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        Self {
            majortype: self.majortype,
            subtype: self.subtype,
            bFixedSizeSamples: self.bFixedSizeSamples,
            bTemporalCompression: self.bTemporalCompression,
            lSampleSize: self.lSampleSize,
            formattype: self.formattype,
            pUnk: self.pUnk.clone(),
            cbFormat: self.cbFormat,
            pbFormat: self.pbFormat,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_MEDIA_TYPE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_PICTURE {
    pub pwszMIMEType: super::super::Foundation::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_PICTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_PICTURE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_PICTURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PICTURE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_PICTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WM_PLAYBACK_DRC_LEVEL = i32;
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = 0i32;
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = 1i32;
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = 2i32;
#[repr(C)]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl ::core::marker::Copy for WM_PORT_NUMBER_RANGE {}
impl ::core::clone::Clone for WM_PORT_NUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_PORT_NUMBER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PORT_NUMBER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_PORT_NUMBER_RANGE {}
impl ::core::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: super::super::Foundation::PWSTR,
    pub wszBrowserUserAgent: super::super::Foundation::PWSTR,
    pub wszBrowserWebPage: super::super::Foundation::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: super::super::Foundation::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_READER_CLIENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_READER_CLIENTINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_CLIENTINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl ::core::marker::Copy for WM_READER_STATISTICS {}
impl ::core::clone::Clone for WM_READER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_READER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_READER_STATISTICS {}
impl ::core::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WM_SFEX_TYPE = i32;
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = 2i32;
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = 4i32;
pub type WM_SF_TYPE = i32;
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = 1i32;
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = 2i32;
pub const WM_SF_DATALOSS: WM_SF_TYPE = 4i32;
#[repr(C, packed(2))]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_STREAM_PRIORITY_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_STREAM_PRIORITY_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_STREAM_PRIORITY_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_PRIORITY_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows::core::GUID,
    pub cbFormat: u32,
}
impl ::core::marker::Copy for WM_STREAM_TYPE_INFO {}
impl ::core::clone::Clone for WM_STREAM_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_STREAM_TYPE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_STREAM_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_TYPE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_STREAM_TYPE_INFO {}
impl ::core::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: super::super::Foundation::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_SYNCHRONISED_LYRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_SYNCHRONISED_LYRICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_SYNCHRONISED_LYRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_SYNCHRONISED_LYRICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_SYNCHRONISED_LYRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_USER_TEXT {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub pwszText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_USER_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_USER_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_USER_TEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_USER_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_TEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_USER_TEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub pwszURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_USER_WEB_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_USER_WEB_URL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_USER_WEB_URL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_USER_WEB_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_WEB_URL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_USER_WEB_URL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_WRITER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS {}
impl ::core::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS_EX {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_WRITER_STATISTICS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS_EX {}
impl ::core::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type _AM_ASFWRITERCONFIG_PARAM = i32;
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = 1i32;
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = 2i32;
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = 3i32;
pub const g_dwWMContentAttributes: u32 = 5u32;
pub const g_dwWMNSCAttributes: u32 = 5u32;
pub const g_dwWMSpecialAttributes: u32 = 20u32;
pub const g_wszASFLeakyBucketPairs: &'static str = "ASFLeakyBucketPairs";
pub const g_wszAllowInterlacedOutput: &'static str = "AllowInterlacedOutput";
pub const g_wszAverageLevel: &'static str = "AverageLevel";
pub const g_wszBufferAverage: &'static str = "Buffer Average";
pub const g_wszComplexity: &'static str = "_COMPLEXITYEX";
pub const g_wszComplexityLive: &'static str = "_COMPLEXITYEXLIVE";
pub const g_wszComplexityMax: &'static str = "_COMPLEXITYEXMAX";
pub const g_wszComplexityOffline: &'static str = "_COMPLEXITYEXOFFLINE";
pub const g_wszDecoderComplexityRequested: &'static str = "_DECODERCOMPLEXITYPROFILE";
pub const g_wszDedicatedDeliveryThread: &'static str = "DedicatedDeliveryThread";
pub const g_wszDeinterlaceMode: &'static str = "DeinterlaceMode";
pub const g_wszDeliverOnReceive: &'static str = "DeliverOnReceive";
pub const g_wszDeviceConformanceTemplate: &'static str = "DeviceConformanceTemplate";
pub const g_wszDynamicRangeControl: &'static str = "DynamicRangeControl";
pub const g_wszEDL: &'static str = "_EDL";
pub const g_wszEarlyDataDelivery: &'static str = "EarlyDataDelivery";
pub const g_wszEnableDiscreteOutput: &'static str = "EnableDiscreteOutput";
pub const g_wszEnableFrameInterpolation: &'static str = "EnableFrameInterpolation";
pub const g_wszEnableWMAProSPDIFOutput: &'static str = "EnableWMAProSPDIFOutput";
pub const g_wszFailSeekOnError: &'static str = "FailSeekOnError";
pub const g_wszFixedFrameRate: &'static str = "FixedFrameRate";
pub const g_wszFold6To2Channels3: &'static str = "Fold6To2Channels3";
pub const g_wszFoldToChannelsTemplate: &'static str = "Fold%luTo%luChannels%lu";
pub const g_wszInitialPatternForInverseTelecine: &'static str = "InitialPatternForInverseTelecine";
pub const g_wszInterlacedCoding: &'static str = "InterlacedCoding";
pub const g_wszIsVBRSupported: &'static str = "_ISVBRSUPPORTED";
pub const g_wszJPEGCompressionQuality: &'static str = "JPEGCompressionQuality";
pub const g_wszJustInTimeDecode: &'static str = "JustInTimeDecode";
pub const g_wszMixedClassMode: &'static str = "MixedClassMode";
pub const g_wszMusicClassMode: &'static str = "MusicClassMode";
pub const g_wszMusicSpeechClassMode: &'static str = "MusicSpeechClassMode";
pub const g_wszNeedsPreviousSample: &'static str = "NeedsPreviousSample";
pub const g_wszNumPasses: &'static str = "_PASSESUSED";
pub const g_wszOriginalSourceFormatTag: &'static str = "_SOURCEFORMATTAG";
pub const g_wszOriginalWaveFormat: &'static str = "_ORIGINALWAVEFORMAT";
pub const g_wszPeakValue: &'static str = "PeakValue";
pub const g_wszPermitSeeksBeyondEndOfStream: &'static str = "PermitSeeksBeyondEndOfStream";
pub const g_wszReloadIndexOnSeek: &'static str = "ReloadIndexOnSeek";
pub const g_wszScrambledAudio: &'static str = "ScrambledAudio";
pub const g_wszSingleOutputBuffer: &'static str = "SingleOutputBuffer";
pub const g_wszSoftwareScaling: &'static str = "SoftwareScaling";
pub const g_wszSourceBufferTime: &'static str = "SourceBufferTime";
pub const g_wszSourceMaxBytesAtOnce: &'static str = "SourceMaxBytesAtOnce";
pub const g_wszSpeakerConfig: &'static str = "SpeakerConfig";
pub const g_wszSpeechCaps: &'static str = "SpeechFormatCap";
pub const g_wszSpeechClassMode: &'static str = "SpeechClassMode";
pub const g_wszStreamLanguage: &'static str = "StreamLanguage";
pub const g_wszStreamNumIndexObjects: &'static str = "StreamNumIndexObjects";
pub const g_wszUsePacketAtSeekPoint: &'static str = "UsePacketAtSeekPoint";
pub const g_wszVBRBitrateMax: &'static str = "_RMAX";
pub const g_wszVBRBufferWindowMax: &'static str = "_BMAX";
pub const g_wszVBREnabled: &'static str = "_VBRENABLED";
pub const g_wszVBRPeak: &'static str = "VBR Peak";
pub const g_wszVBRQuality: &'static str = "_VBRQUALITY";
pub const g_wszVideoSampleDurations: &'static str = "VideoSampleDurations";
pub const g_wszWMADID: &'static str = "WM/ADID";
pub const g_wszWMASFPacketCount: &'static str = "WM/ASFPacketCount";
pub const g_wszWMASFSecurityObjectsSize: &'static str = "WM/ASFSecurityObjectsSize";
pub const g_wszWMAlbumArtist: &'static str = "WM/AlbumArtist";
pub const g_wszWMAlbumArtistSort: &'static str = "WM/AlbumArtistSort";
pub const g_wszWMAlbumCoverURL: &'static str = "WM/AlbumCoverURL";
pub const g_wszWMAlbumTitle: &'static str = "WM/AlbumTitle";
pub const g_wszWMAlbumTitleSort: &'static str = "WM/AlbumTitleSort";
pub const g_wszWMAspectRatioX: &'static str = "AspectRatioX";
pub const g_wszWMAspectRatioY: &'static str = "AspectRatioY";
pub const g_wszWMAudioFileURL: &'static str = "WM/AudioFileURL";
pub const g_wszWMAudioSourceURL: &'static str = "WM/AudioSourceURL";
pub const g_wszWMAuthor: &'static str = "Author";
pub const g_wszWMAuthorSort: &'static str = "AuthorSort";
pub const g_wszWMAuthorURL: &'static str = "WM/AuthorURL";
pub const g_wszWMBannerImageData: &'static str = "BannerImageData";
pub const g_wszWMBannerImageType: &'static str = "BannerImageType";
pub const g_wszWMBannerImageURL: &'static str = "BannerImageURL";
pub const g_wszWMBeatsPerMinute: &'static str = "WM/BeatsPerMinute";
pub const g_wszWMBitrate: &'static str = "Bitrate";
pub const g_wszWMBroadcast: &'static str = "Broadcast";
pub const g_wszWMCategory: &'static str = "WM/Category";
pub const g_wszWMCodec: &'static str = "WM/Codec";
pub const g_wszWMComposer: &'static str = "WM/Composer";
pub const g_wszWMComposerSort: &'static str = "WM/ComposerSort";
pub const g_wszWMConductor: &'static str = "WM/Conductor";
pub const g_wszWMContainerFormat: &'static str = "WM/ContainerFormat";
pub const g_wszWMContentDistributor: &'static str = "WM/ContentDistributor";
pub const g_wszWMContentGroupDescription: &'static str = "WM/ContentGroupDescription";
pub const g_wszWMCopyright: &'static str = "Copyright";
pub const g_wszWMCopyrightURL: &'static str = "CopyrightURL";
pub const g_wszWMCurrentBitrate: &'static str = "CurrentBitrate";
pub const g_wszWMDRM: &'static str = "WM/DRM";
pub const g_wszWMDRM_ContentID: &'static str = "DRM_ContentID";
pub const g_wszWMDRM_Flags: &'static str = "DRM_Flags";
pub const g_wszWMDRM_HeaderSignPrivKey: &'static str = "DRM_HeaderSignPrivKey";
pub const g_wszWMDRM_IndividualizedVersion: &'static str = "DRM_IndividualizedVersion";
pub const g_wszWMDRM_KeyID: &'static str = "DRM_KeyID";
pub const g_wszWMDRM_KeySeed: &'static str = "DRM_KeySeed";
pub const g_wszWMDRM_LASignatureCert: &'static str = "DRM_LASignatureCert";
pub const g_wszWMDRM_LASignatureLicSrvCert: &'static str = "DRM_LASignatureLicSrvCert";
pub const g_wszWMDRM_LASignaturePrivKey: &'static str = "DRM_LASignaturePrivKey";
pub const g_wszWMDRM_LASignatureRootCert: &'static str = "DRM_LASignatureRootCert";
pub const g_wszWMDRM_Level: &'static str = "DRM_Level";
pub const g_wszWMDRM_LicenseAcqURL: &'static str = "DRM_LicenseAcqURL";
pub const g_wszWMDRM_SourceID: &'static str = "DRM_SourceID";
pub const g_wszWMDRM_V1LicenseAcqURL: &'static str = "DRM_V1LicenseAcqURL";
pub const g_wszWMDVDID: &'static str = "WM/DVDID";
pub const g_wszWMDescription: &'static str = "Description";
pub const g_wszWMDirector: &'static str = "WM/Director";
pub const g_wszWMDuration: &'static str = "Duration";
pub const g_wszWMEncodedBy: &'static str = "WM/EncodedBy";
pub const g_wszWMEncodingSettings: &'static str = "WM/EncodingSettings";
pub const g_wszWMEncodingTime: &'static str = "WM/EncodingTime";
pub const g_wszWMEpisodeNumber: &'static str = "WM/EpisodeNumber";
pub const g_wszWMFileSize: &'static str = "FileSize";
pub const g_wszWMGenre: &'static str = "WM/Genre";
pub const g_wszWMGenreID: &'static str = "WM/GenreID";
pub const g_wszWMHasArbitraryDataStream: &'static str = "HasArbitraryDataStream";
pub const g_wszWMHasAttachedImages: &'static str = "HasAttachedImages";
pub const g_wszWMHasAudio: &'static str = "HasAudio";
pub const g_wszWMHasFileTransferStream: &'static str = "HasFileTransferStream";
pub const g_wszWMHasImage: &'static str = "HasImage";
pub const g_wszWMHasScript: &'static str = "HasScript";
pub const g_wszWMHasVideo: &'static str = "HasVideo";
pub const g_wszWMISAN: &'static str = "WM/ISAN";
pub const g_wszWMISRC: &'static str = "WM/ISRC";
pub const g_wszWMInitialKey: &'static str = "WM/InitialKey";
pub const g_wszWMIsCompilation: &'static str = "WM/IsCompilation";
pub const g_wszWMIsVBR: &'static str = "IsVBR";
pub const g_wszWMLanguage: &'static str = "WM/Language";
pub const g_wszWMLyrics: &'static str = "WM/Lyrics";
pub const g_wszWMLyrics_Synchronised: &'static str = "WM/Lyrics_Synchronised";
pub const g_wszWMMCDI: &'static str = "WM/MCDI";
pub const g_wszWMMediaClassPrimaryID: &'static str = "WM/MediaClassPrimaryID";
pub const g_wszWMMediaClassSecondaryID: &'static str = "WM/MediaClassSecondaryID";
pub const g_wszWMMediaCredits: &'static str = "WM/MediaCredits";
pub const g_wszWMMediaIsDelay: &'static str = "WM/MediaIsDelay";
pub const g_wszWMMediaIsFinale: &'static str = "WM/MediaIsFinale";
pub const g_wszWMMediaIsLive: &'static str = "WM/MediaIsLive";
pub const g_wszWMMediaIsPremiere: &'static str = "WM/MediaIsPremiere";
pub const g_wszWMMediaIsRepeat: &'static str = "WM/MediaIsRepeat";
pub const g_wszWMMediaIsSAP: &'static str = "WM/MediaIsSAP";
pub const g_wszWMMediaIsStereo: &'static str = "WM/MediaIsStereo";
pub const g_wszWMMediaIsSubtitled: &'static str = "WM/MediaIsSubtitled";
pub const g_wszWMMediaIsTape: &'static str = "WM/MediaIsTape";
pub const g_wszWMMediaNetworkAffiliation: &'static str = "WM/MediaNetworkAffiliation";
pub const g_wszWMMediaOriginalBroadcastDateTime: &'static str = "WM/MediaOriginalBroadcastDateTime";
pub const g_wszWMMediaOriginalChannel: &'static str = "WM/MediaOriginalChannel";
pub const g_wszWMMediaStationCallSign: &'static str = "WM/MediaStationCallSign";
pub const g_wszWMMediaStationName: &'static str = "WM/MediaStationName";
pub const g_wszWMModifiedBy: &'static str = "WM/ModifiedBy";
pub const g_wszWMMood: &'static str = "WM/Mood";
pub const g_wszWMNSCAddress: &'static str = "NSC_Address";
pub const g_wszWMNSCDescription: &'static str = "NSC_Description";
pub const g_wszWMNSCEmail: &'static str = "NSC_Email";
pub const g_wszWMNSCName: &'static str = "NSC_Name";
pub const g_wszWMNSCPhone: &'static str = "NSC_Phone";
pub const g_wszWMNumberOfFrames: &'static str = "NumberOfFrames";
pub const g_wszWMOptimalBitrate: &'static str = "OptimalBitrate";
pub const g_wszWMOriginalAlbumTitle: &'static str = "WM/OriginalAlbumTitle";
pub const g_wszWMOriginalArtist: &'static str = "WM/OriginalArtist";
pub const g_wszWMOriginalFilename: &'static str = "WM/OriginalFilename";
pub const g_wszWMOriginalLyricist: &'static str = "WM/OriginalLyricist";
pub const g_wszWMOriginalReleaseTime: &'static str = "WM/OriginalReleaseTime";
pub const g_wszWMOriginalReleaseYear: &'static str = "WM/OriginalReleaseYear";
pub const g_wszWMParentalRating: &'static str = "WM/ParentalRating";
pub const g_wszWMParentalRatingReason: &'static str = "WM/ParentalRatingReason";
pub const g_wszWMPartOfSet: &'static str = "WM/PartOfSet";
pub const g_wszWMPeakBitrate: &'static str = "WM/PeakBitrate";
pub const g_wszWMPeriod: &'static str = "WM/Period";
pub const g_wszWMPicture: &'static str = "WM/Picture";
pub const g_wszWMPlaylistDelay: &'static str = "WM/PlaylistDelay";
pub const g_wszWMProducer: &'static str = "WM/Producer";
pub const g_wszWMPromotionURL: &'static str = "WM/PromotionURL";
pub const g_wszWMProtected: &'static str = "Is_Protected";
pub const g_wszWMProtectionType: &'static str = "WM/ProtectionType";
pub const g_wszWMProvider: &'static str = "WM/Provider";
pub const g_wszWMProviderCopyright: &'static str = "WM/ProviderCopyright";
pub const g_wszWMProviderRating: &'static str = "WM/ProviderRating";
pub const g_wszWMProviderStyle: &'static str = "WM/ProviderStyle";
pub const g_wszWMPublisher: &'static str = "WM/Publisher";
pub const g_wszWMRadioStationName: &'static str = "WM/RadioStationName";
pub const g_wszWMRadioStationOwner: &'static str = "WM/RadioStationOwner";
pub const g_wszWMRating: &'static str = "Rating";
pub const g_wszWMSeasonNumber: &'static str = "WM/SeasonNumber";
pub const g_wszWMSeekable: &'static str = "Seekable";
pub const g_wszWMSharedUserRating: &'static str = "WM/SharedUserRating";
pub const g_wszWMSignature_Name: &'static str = "Signature_Name";
pub const g_wszWMSkipBackward: &'static str = "Can_Skip_Backward";
pub const g_wszWMSkipForward: &'static str = "Can_Skip_Forward";
pub const g_wszWMStreamTypeInfo: &'static str = "WM/StreamTypeInfo";
pub const g_wszWMStridable: &'static str = "Stridable";
pub const g_wszWMSubTitle: &'static str = "WM/SubTitle";
pub const g_wszWMSubTitleDescription: &'static str = "WM/SubTitleDescription";
pub const g_wszWMSubscriptionContentID: &'static str = "WM/SubscriptionContentID";
pub const g_wszWMText: &'static str = "WM/Text";
pub const g_wszWMTitle: &'static str = "Title";
pub const g_wszWMTitleSort: &'static str = "TitleSort";
pub const g_wszWMToolName: &'static str = "WM/ToolName";
pub const g_wszWMToolVersion: &'static str = "WM/ToolVersion";
pub const g_wszWMTrack: &'static str = "WM/Track";
pub const g_wszWMTrackNumber: &'static str = "WM/TrackNumber";
pub const g_wszWMTrusted: &'static str = "Is_Trusted";
pub const g_wszWMUniqueFileIdentifier: &'static str = "WM/UniqueFileIdentifier";
pub const g_wszWMUse_Advanced_DRM: &'static str = "Use_Advanced_DRM";
pub const g_wszWMUse_DRM: &'static str = "Use_DRM";
pub const g_wszWMUserWebURL: &'static str = "WM/UserWebURL";
pub const g_wszWMVideoClosedCaptioning: &'static str = "WM/VideoClosedCaptioning";
pub const g_wszWMVideoFrameRate: &'static str = "WM/VideoFrameRate";
pub const g_wszWMVideoHeight: &'static str = "WM/VideoHeight";
pub const g_wszWMVideoWidth: &'static str = "WM/VideoWidth";
pub const g_wszWMWMADRCAverageReference: &'static str = "WM/WMADRCAverageReference";
pub const g_wszWMWMADRCAverageTarget: &'static str = "WM/WMADRCAverageTarget";
pub const g_wszWMWMADRCPeakReference: &'static str = "WM/WMADRCPeakReference";
pub const g_wszWMWMADRCPeakTarget: &'static str = "WM/WMADRCPeakTarget";
pub const g_wszWMWMCPDistributor: &'static str = "WM/WMCPDistributor";
pub const g_wszWMWMCPDistributorID: &'static str = "WM/WMCPDistributorID";
pub const g_wszWMWMCollectionGroupID: &'static str = "WM/WMCollectionGroupID";
pub const g_wszWMWMCollectionID: &'static str = "WM/WMCollectionID";
pub const g_wszWMWMContentID: &'static str = "WM/WMContentID";
pub const g_wszWMWMShadowFileSourceDRMType: &'static str = "WM/WMShadowFileSourceDRMType";
pub const g_wszWMWMShadowFileSourceFileType: &'static str = "WM/WMShadowFileSourceFileType";
pub const g_wszWMWriter: &'static str = "WM/Writer";
pub const g_wszWMYear: &'static str = "WM/Year";
pub const g_wszWatermarkCLSID: &'static str = "WatermarkCLSID";
pub const g_wszWatermarkConfig: &'static str = "WatermarkConfig";
