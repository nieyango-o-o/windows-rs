#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFrame(::windows::core::IUnknown);
impl Direct3D11CaptureFrame {
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Surface(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn ContentSize(&self) -> ::windows::core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SizeInt32>(result__)
        }
    }
}
impl ::core::clone::Clone for Direct3D11CaptureFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Direct3D11CaptureFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Direct3D11CaptureFrame {}
unsafe impl ::windows::core::RuntimeType for Direct3D11CaptureFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFrame;{fa50c623-38da-4b32-acf3-fa9734ad800e})");
}
unsafe impl ::windows::core::Interface for Direct3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa50c623_38da_4b32_acf3_fa9734ad800e);
}
impl ::windows::core::RuntimeName for Direct3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
}
impl ::core::convert::From<Direct3D11CaptureFrame> for ::windows::core::IUnknown {
    fn from(value: Direct3D11CaptureFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Direct3D11CaptureFrame> for ::windows::core::IUnknown {
    fn from(value: &Direct3D11CaptureFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Direct3D11CaptureFrame> for ::windows::core::IInspectable {
    fn from(value: Direct3D11CaptureFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Direct3D11CaptureFrame> for ::windows::core::IInspectable {
    fn from(value: &Direct3D11CaptureFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Direct3D11CaptureFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &Direct3D11CaptureFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Direct3D11CaptureFrame {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFrame {}
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct Direct3D11CaptureFramePool(::windows::core::IUnknown);
impl Direct3D11CaptureFramePool {
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Graphics_DirectX', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn Recreate<'a, Param0: ::windows::core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::core::IntoParam<'a, super::SizeInt32>>(&self, device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn TryGetNextFrame(&self) -> ::windows::core::Result<Direct3D11CaptureFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Direct3D11CaptureFrame>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn CreateCaptureSession<'a, Param0: ::windows::core::IntoParam<'a, GraphicsCaptureItem>>(&self, item: Param0) -> ::windows::core::Result<GraphicsCaptureSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureSession>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'System'*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Graphics_DirectX', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::core::IntoParam<'a, super::SizeInt32>>(device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::core::Result<Direct3D11CaptureFramePool> {
        Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi(), &mut result__).from_abi::<Direct3D11CaptureFramePool>(result__)
        })
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Graphics_DirectX', 'Graphics_DirectX_Direct3D11'*"]
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateFreeThreaded<'a, Param0: ::windows::core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>, Param3: ::windows::core::IntoParam<'a, super::SizeInt32>>(device: Param0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: Param3) -> ::windows::core::Result<Direct3D11CaptureFramePool> {
        Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), device.into_param().abi(), pixelformat, numberofbuffers, size.into_param().abi(), &mut result__).from_abi::<Direct3D11CaptureFramePool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Direct3D11CaptureFramePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Direct3D11CaptureFramePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Direct3D11CaptureFramePool {}
unsafe impl ::windows::core::RuntimeType for Direct3D11CaptureFramePool {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.Direct3D11CaptureFramePool;{24eb6d22-1975-422e-82e7-780dbd8ddf24})");
}
unsafe impl ::windows::core::Interface for Direct3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePoolVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24eb6d22_1975_422e_82e7_780dbd8ddf24);
}
impl ::windows::core::RuntimeName for Direct3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
}
impl ::core::convert::From<Direct3D11CaptureFramePool> for ::windows::core::IUnknown {
    fn from(value: Direct3D11CaptureFramePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Direct3D11CaptureFramePool> for ::windows::core::IUnknown {
    fn from(value: &Direct3D11CaptureFramePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Direct3D11CaptureFramePool> for ::windows::core::IInspectable {
    fn from(value: Direct3D11CaptureFramePool) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Direct3D11CaptureFramePool> for ::windows::core::IInspectable {
    fn from(value: &Direct3D11CaptureFramePool) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Direct3D11CaptureFramePool) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Direct3D11CaptureFramePool> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Direct3D11CaptureFramePool) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &Direct3D11CaptureFramePool {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Direct3D11CaptureFramePool {}
unsafe impl ::core::marker::Sync for Direct3D11CaptureFramePool {}
#[doc = "*Required features: 'Graphics_Capture'*"]
pub struct GraphicsCaptureAccess {}
impl GraphicsCaptureAccess {
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation', 'Security_Authorization_AppCapabilityAccess'*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))]
    pub fn RequestAccessAsync(request: GraphicsCaptureAccessKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>> {
        Self::IGraphicsCaptureAccessStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureAccessStatics<R, F: FnOnce(&IGraphicsCaptureAccessStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GraphicsCaptureAccess, IGraphicsCaptureAccessStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GraphicsCaptureAccess {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureAccess";
}
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl ::core::marker::Copy for GraphicsCaptureAccessKind {}
impl ::core::clone::Clone for GraphicsCaptureAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for GraphicsCaptureAccessKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GraphicsCaptureAccessKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCaptureAccessKind {}
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureAccessKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Capture.GraphicsCaptureAccessKind;i4)");
}
impl ::windows::core::DefaultType for GraphicsCaptureAccessKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct GraphicsCaptureItem(::windows::core::IUnknown);
impl GraphicsCaptureItem {
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn Size(&self) -> ::windows::core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CreateFromVisual<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Composition::Visual>>(visual: Param0) -> ::windows::core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), visual.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    #[doc = "*Required features: 'Graphics_Capture', 'UI'*"]
    #[cfg(feature = "UI")]
    pub fn TryCreateFromWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::WindowId>>(windowid: Param0) -> ::windows::core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), windowid.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn TryCreateFromDisplayId<'a, Param0: ::windows::core::IntoParam<'a, super::DisplayId>>(displayid: Param0) -> ::windows::core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), displayid.into_param().abi(), &mut result__).from_abi::<GraphicsCaptureItem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GraphicsCaptureItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GraphicsCaptureItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCaptureItem {}
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureItem;{79c3f95b-31f7-4ec2-a464-632ef5d30760})");
}
unsafe impl ::windows::core::Interface for GraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
}
impl ::windows::core::RuntimeName for GraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
}
impl ::core::convert::From<GraphicsCaptureItem> for ::windows::core::IUnknown {
    fn from(value: GraphicsCaptureItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCaptureItem> for ::windows::core::IUnknown {
    fn from(value: &GraphicsCaptureItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GraphicsCaptureItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GraphicsCaptureItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GraphicsCaptureItem> for ::windows::core::IInspectable {
    fn from(value: GraphicsCaptureItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCaptureItem> for ::windows::core::IInspectable {
    fn from(value: &GraphicsCaptureItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GraphicsCaptureItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GraphicsCaptureItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GraphicsCaptureItem {}
unsafe impl ::core::marker::Sync for GraphicsCaptureItem {}
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct GraphicsCapturePicker(::windows::core::IUnknown);
impl GraphicsCapturePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GraphicsCapturePicker, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleItemAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for GraphicsCapturePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GraphicsCapturePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCapturePicker {}
unsafe impl ::windows::core::RuntimeType for GraphicsCapturePicker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCapturePicker;{5a1711b3-ad79-4b4a-9336-1318fdde3539})");
}
unsafe impl ::windows::core::Interface for GraphicsCapturePicker {
    type Vtable = IGraphicsCapturePickerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a1711b3_ad79_4b4a_9336_1318fdde3539);
}
impl ::windows::core::RuntimeName for GraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCapturePicker";
}
impl ::core::convert::From<GraphicsCapturePicker> for ::windows::core::IUnknown {
    fn from(value: GraphicsCapturePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCapturePicker> for ::windows::core::IUnknown {
    fn from(value: &GraphicsCapturePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GraphicsCapturePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GraphicsCapturePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GraphicsCapturePicker> for ::windows::core::IInspectable {
    fn from(value: GraphicsCapturePicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCapturePicker> for ::windows::core::IInspectable {
    fn from(value: &GraphicsCapturePicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GraphicsCapturePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GraphicsCapturePicker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GraphicsCapturePicker {}
unsafe impl ::core::marker::Sync for GraphicsCapturePicker {}
#[doc = "*Required features: 'Graphics_Capture'*"]
#[repr(transparent)]
pub struct GraphicsCaptureSession(::windows::core::IUnknown);
impl GraphicsCaptureSession {
    #[doc = "*Required features: 'Graphics_Capture', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn StartCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn IsCursorCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn IsBorderRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn SetIsBorderRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Graphics_Capture'*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IGraphicsCaptureSessionStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GraphicsCaptureSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GraphicsCaptureSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GraphicsCaptureSession {}
unsafe impl ::windows::core::RuntimeType for GraphicsCaptureSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Capture.GraphicsCaptureSession;{814e42a9-f70f-4ad7-939b-fddcc6eb880d})");
}
unsafe impl ::windows::core::Interface for GraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814e42a9_f70f_4ad7_939b_fddcc6eb880d);
}
impl ::windows::core::RuntimeName for GraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
}
impl ::core::convert::From<GraphicsCaptureSession> for ::windows::core::IUnknown {
    fn from(value: GraphicsCaptureSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCaptureSession> for ::windows::core::IUnknown {
    fn from(value: &GraphicsCaptureSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GraphicsCaptureSession> for ::windows::core::IInspectable {
    fn from(value: GraphicsCaptureSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GraphicsCaptureSession> for ::windows::core::IInspectable {
    fn from(value: &GraphicsCaptureSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GraphicsCaptureSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GraphicsCaptureSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GraphicsCaptureSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &GraphicsCaptureSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GraphicsCaptureSession {}
unsafe impl ::core::marker::Sync for GraphicsCaptureSession {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa50c623_38da_4b32_acf3_fa9734ad800e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePool(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePoolVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24eb6d22_1975_422e_82e7_780dbd8ddf24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePoolStatics {
    type Vtable = IDirect3D11CaptureFramePoolStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7784056a_67aa_4d53_ae54_1088d5a8ca21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDirect3D11CaptureFramePoolStatics2 {
    type Vtable = IDirect3D11CaptureFramePoolStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x589b103f_6bbc_5df5_a991_02e28b3b66d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D11CaptureFramePoolStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureAccessStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureAccessStatics {
    type Vtable = IGraphicsCaptureAccessStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743ed370_06ec_5040_a58a_901f0f757095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureAccessStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: GraphicsCaptureAccessKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authorization_AppCapabilityAccess")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemStatics {
    type Vtable = IGraphicsCaptureItemStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87ebea5_457c_5788_ab47_0cf1d3637e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureItemStatics2 {
    type Vtable = IGraphicsCaptureItemStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b92acc9_e584_5862_bf5c_9c316c6d2dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windowid: super::super::UI::WindowId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayid: super::DisplayId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCapturePicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCapturePicker {
    type Vtable = IGraphicsCapturePickerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a1711b3_ad79_4b4a_9336_1318fdde3539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCapturePickerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x814e42a9_f70f_4ad7_939b_fddcc6eb880d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession2 {
    type Vtable = IGraphicsCaptureSession2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c39ae40_7d2e_5044_804e_8b6799d4cf9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSession3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureSession3 {
    type Vtable = IGraphicsCaptureSession3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2cdd966_22ae_5ea1_9596_3a289344c3be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSession3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IGraphicsCaptureSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGraphicsCaptureSessionStatics {
    type Vtable = IGraphicsCaptureSessionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2224a540_5974_49aa_b232_0882536f4cb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureSessionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
