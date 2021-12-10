#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(transparent)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ActivationKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ActivationKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationKind {}
unsafe impl ::windows::core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
impl ::windows::core::DefaultType for ActivationKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ApplicationExecutionState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ApplicationExecutionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationExecutionState {}
unsafe impl ::windows::core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
impl ::windows::core::DefaultType for ApplicationExecutionState {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
impl ::windows::core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderAddAppointmentActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
impl ::windows::core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderRemoveAppointmentActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
impl ::windows::core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderReplaceAppointmentActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderShowTimeFrameActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows::core::IUnknown);
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
}
unsafe impl ::windows::core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
impl ::windows::core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundActivatedEventArgs> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundActivatedEventArgs> {
        ::core::convert::TryInto::<IBackgroundActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
impl ::windows::core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::core::convert::TryInto::<IBarcodeScannerPreviewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
impl ::windows::core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::core::convert::TryInto::<ICachedFileUpdaterActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraSettingsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
}
unsafe impl ::windows::core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
impl ::windows::core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<ICameraSettingsActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(::windows::core::IUnknown);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
}
unsafe impl ::windows::core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
impl ::windows::core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommandLineActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICommandLineActivatedEventArgs> {
        ::core::convert::TryInto::<ICommandLineActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[repr(transparent)]
pub struct CommandLineActivationOperation(::windows::core::IUnknown);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivationOperation {}
unsafe impl ::windows::core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
}
unsafe impl ::windows::core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
impl ::windows::core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
}
unsafe impl ::windows::core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
impl ::windows::core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactCallActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactCallActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactCallActivatedEventArgs> {
        ::core::convert::TryInto::<IContactCallActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(::windows::core::IUnknown);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMapActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
}
unsafe impl ::windows::core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
impl ::windows::core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMapActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMapActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMapActivatedEventArgs> {
        ::core::convert::TryInto::<IContactMapActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMessageActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
}
unsafe impl ::windows::core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
impl ::windows::core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMessageActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMessageActivatedEventArgs> {
        ::core::convert::TryInto::<IContactMessageActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
}
unsafe impl ::windows::core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
impl ::windows::core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPanelActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPanelActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPanelActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
}
unsafe impl ::windows::core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
impl ::windows::core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPickerActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPickerActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPickerActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(::windows::core::IUnknown);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPostActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
}
unsafe impl ::windows::core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
impl ::windows::core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPostActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPostActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPostActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPostActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactVideoCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
}
unsafe impl ::windows::core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
impl ::windows::core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::core::convert::TryInto::<IContactVideoCallActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(::windows::core::IUnknown);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
}
unsafe impl ::windows::core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
impl ::windows::core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDeviceActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDeviceActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDeviceActivatedEventArgs> {
        ::core::convert::TryInto::<IDeviceActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
}
unsafe impl ::windows::core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
impl ::windows::core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDevicePairingActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::core::convert::TryInto::<IDevicePairingActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialReceiverActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
}
unsafe impl ::windows::core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
impl ::windows::core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDialReceiverActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::core::convert::TryInto::<IDialReceiverActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[repr(transparent)]
pub struct FileActivatedEventArgs(::windows::core::IUnknown);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for FileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
}
unsafe impl ::windows::core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
impl ::windows::core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithCallerPackageFamilyName>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithNeighboringFiles>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
}
unsafe impl ::windows::core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
impl ::windows::core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for FileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
}
unsafe impl ::windows::core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
impl ::windows::core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFileOpenPickerContinuationEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
}
unsafe impl ::windows::core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
impl ::windows::core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(::windows::core::IUnknown);
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for FileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
}
unsafe impl ::windows::core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
impl ::windows::core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFileSavePickerContinuationEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(::windows::core::IUnknown);
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
}
impl ::core::clone::Clone for FolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderPickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
}
unsafe impl ::windows::core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
impl ::windows::core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFolderPickerContinuationEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFolderPickerContinuationEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[repr(transparent)]
pub struct IActivatedEventArgs(::windows::core::IUnknown);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
}
unsafe impl ::windows::core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(::windows::core::IUnknown);
impl IActivatedEventArgsWithUser {
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IActivatedEventArgsWithUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgsWithUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsWithUser {}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
}
unsafe impl ::windows::core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUserVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUserVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(::windows::core::IUnknown);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IApplicationViewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApplicationViewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationViewActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
}
unsafe impl ::windows::core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(::windows::core::IUnknown);
impl IBackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
}
unsafe impl ::windows::core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
);
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
}
unsafe impl ::windows::core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl ICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICachedFileUpdaterActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
);
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraSettingsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
}
unsafe impl ::windows::core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(::windows::core::IUnknown);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandLineActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
}
unsafe impl ::windows::core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandLineActivationOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
pub struct IContactActivatedEventArgs(::windows::core::IUnknown);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
}
unsafe impl ::windows::core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
}
unsafe impl ::windows::core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(::windows::core::IUnknown);
impl IContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMapActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
}
unsafe impl ::windows::core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMessageActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
}
unsafe impl ::windows::core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl IContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPanelActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
}
unsafe impl ::windows::core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl IContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
}
unsafe impl ::windows::core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))] usize,
);
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(::windows::core::IUnknown);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPostActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
}
unsafe impl ::windows::core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactVideoCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
}
unsafe impl ::windows::core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContactsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactsProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
}
unsafe impl ::windows::core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(::windows::core::IUnknown);
impl IContinuationActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContinuationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContinuationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinuationActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
}
unsafe impl ::windows::core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(::windows::core::IUnknown);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
}
unsafe impl ::windows::core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl IDevicePairingActivatedEventArgs {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
}
unsafe impl ::windows::core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialReceiverActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
}
unsafe impl ::windows::core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFileActivatedEventArgs(::windows::core::IUnknown);
impl IFileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithCallerPackageFamilyName {}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyNameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithNeighboringFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithNeighboringFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithNeighboringFiles {}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFilesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFilesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
);
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs2 {}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
impl IFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
}
unsafe impl ::windows::core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs2 {}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(::windows::core::IUnknown);
impl IFileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
}
unsafe impl ::windows::core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(::windows::core::IUnknown);
impl IFolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFolderPickerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
}
unsafe impl ::windows::core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs2 {}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
}
unsafe impl ::windows::core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
}
unsafe impl ::windows::core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))] usize,
);
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhoneCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
}
unsafe impl ::windows::core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPickerReturnedActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
}
unsafe impl ::windows::core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(::windows::core::IUnknown);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPrelaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrelaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrelaunchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
}
unsafe impl ::windows::core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl IPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint3DWorkflowActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl IPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskSettingsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
}
unsafe impl ::windows::core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(::windows::core::IUnknown);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolForResultsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
}
unsafe impl ::windows::core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedLaunchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
}
unsafe impl ::windows::core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(::windows::core::IUnknown);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(::windows::core::IUnknown);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgsWithLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgsWithLinguisticDetails {}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))] usize,
);
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl IShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareTargetActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
}
unsafe impl ::windows::core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplashScreen(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplashScreen {
    type Vtable = ISplashScreenVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreenVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStartupTaskActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
}
unsafe impl ::windows::core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileActivatedInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))] usize,
);
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
}
unsafe impl ::windows::core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IUserDataAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
}
unsafe impl ::windows::core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))] usize,
);
#[repr(transparent)]
pub struct IViewSwitcherProvider(::windows::core::IUnknown);
impl IViewSwitcherProvider {
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IViewSwitcherProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewSwitcherProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewSwitcherProvider {}
unsafe impl ::windows::core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
}
unsafe impl ::windows::core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl IVoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVoiceCommandActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
}
unsafe impl ::windows::core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_SpeechRecognition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(::windows::core::IUnknown);
impl IWalletActionActivatedEventArgs {
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWalletActionActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
}
unsafe impl ::windows::core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
}
unsafe impl ::windows::core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Provider")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))] usize,
);
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
}
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
);
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(::windows::core::IUnknown);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
}
unsafe impl ::windows::core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
impl ::windows::core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs2> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs2> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrelaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IPrelaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
}
unsafe impl ::windows::core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
impl ::windows::core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenActivatedEventArgs> {
        ::core::convert::TryInto::<ILockScreenActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
}
unsafe impl ::windows::core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
impl ::windows::core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::core::convert::TryInto::<ILockScreenCallActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenComponentActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
}
unsafe impl ::windows::core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
impl ::windows::core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
}
unsafe impl ::windows::core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
impl ::windows::core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPhoneCallActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::core::convert::TryInto::<IPhoneCallActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerReturnedActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
}
unsafe impl ::windows::core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
impl ::windows::core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::core::convert::TryInto::<IPickerReturnedActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
}
impl ::core::clone::Clone for Print3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
}
unsafe impl ::windows::core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
impl ::windows::core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::core::convert::TryInto::<IPrint3DWorkflowActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
}
unsafe impl ::windows::core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
impl ::windows::core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<IPrintTaskSettingsActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
}
unsafe impl ::windows::core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
impl ::windows::core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
}
unsafe impl ::windows::core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
impl ::windows::core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolForResultsActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for RestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
}
unsafe impl ::windows::core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
impl ::windows::core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IRestrictedLaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[repr(transparent)]
pub struct SearchActivatedEventArgs(::windows::core::IUnknown);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
}
unsafe impl ::windows::core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
impl ::windows::core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgs> {
        ::core::convert::TryInto::<ISearchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::core::convert::TryInto::<ISearchActivatedEventArgsWithLinguisticDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for ShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareTargetActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
}
unsafe impl ::windows::core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
impl ::windows::core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IShareTargetActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IShareTargetActivatedEventArgs> {
        ::core::convert::TryInto::<IShareTargetActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[repr(transparent)]
pub struct SplashScreen(::windows::core::IUnknown);
impl SplashScreen {
    #[cfg(feature = "Foundation")]
    pub fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SplashScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplashScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplashScreen {}
unsafe impl ::windows::core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
}
unsafe impl ::windows::core::Interface for SplashScreen {
    type Vtable = ISplashScreenVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
impl ::windows::core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IUnknown {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IInspectable {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTaskActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
}
unsafe impl ::windows::core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
impl ::windows::core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStartupTaskActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::core::convert::TryInto::<IStartupTaskActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[repr(transparent)]
pub struct TileActivatedInfo(::windows::core::IUnknown);
impl TileActivatedInfo {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>(result__)
        }
    }
}
impl ::core::clone::Clone for TileActivatedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileActivatedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileActivatedInfo {}
unsafe impl ::windows::core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
}
unsafe impl ::windows::core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
impl ::windows::core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
}
unsafe impl ::windows::core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
impl ::windows::core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IToastNotificationActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::core::convert::TryInto::<IToastNotificationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
impl ::windows::core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IUserDataAccountProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
}
unsafe impl ::windows::core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
impl ::windows::core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::core::convert::TryInto::<IVoiceCommandActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(::windows::core::IUnknown);
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletActionActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
}
unsafe impl ::windows::core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
impl ::windows::core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWalletActionActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWalletActionActivatedEventArgs> {
        ::core::convert::TryInto::<IWalletActionActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderActivatedEventArgs {}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
}
unsafe impl ::windows::core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
impl ::windows::core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IWebAccountProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
}
unsafe impl ::windows::core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
impl ::windows::core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::core::convert::TryInto::<IWebAuthenticationBrokerContinuationEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
