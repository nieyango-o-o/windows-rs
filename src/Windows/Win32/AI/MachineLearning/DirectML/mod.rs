#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, T: ::windows::core::Interface>(d3d12device: Param0, flags: DML_CREATE_DEVICE_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMLCreateDevice(d3d12device: ::windows::core::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DMLCreateDevice(d3d12device.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice1<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12Device>, T: ::windows::core::Interface>(d3d12device: Param0, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMLCreateDevice1(d3d12device: ::windows::core::RawPtr, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DMLCreateDevice1(d3d12device.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(minimumfeaturelevel), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct DML_ACTIVATION_CELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_CELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_CELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_ELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_ELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_HARDMAX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_IDENTITY_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_LINEAR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SlopeTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_RELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_RELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Gamma: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Bias: f32,
    pub Threshold: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SHRINK_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SIGMOID_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SOFTMAX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Steepness: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_TANH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_TANH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    pub InputParametersTensor: *mut DML_TENSOR_DESC,
    pub InputFirstMomentTensor: *mut DML_TENSOR_DESC,
    pub InputSecondMomentTensor: *mut DML_TENSOR_DESC,
    pub GradientTensor: *mut DML_TENSOR_DESC,
    pub TrainingStepTensor: *mut DML_TENSOR_DESC,
    pub OutputParametersTensor: *mut DML_TENSOR_DESC,
    pub OutputFirstMomentTensor: *mut DML_TENSOR_DESC,
    pub OutputSecondMomentTensor: *mut DML_TENSOR_DESC,
    pub LearningRate: f32,
    pub Beta1: f32,
    pub Beta2: f32,
    pub Epsilon: f32,
}
impl ::core::marker::Copy for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ADAM_OPTIMIZER_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::default::Default for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ARGMAX_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ARGMAX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ARGMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ARGMAX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ARGMIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ARGMIN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ARGMIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ARGMIN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_AVERAGE_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_AVERAGE_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_AVERAGE_POOLING_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_AXIS_DIRECTION = i32;
pub const DML_AXIS_DIRECTION_INCREASING: DML_AXIS_DIRECTION = 0i32;
pub const DML_AXIS_DIRECTION_DECREASING: DML_AXIS_DIRECTION = 1i32;
#[repr(C)]
pub struct DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub MeanTensor: *mut DML_TENSOR_DESC,
    pub VarianceTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputBiasGradientTensor: *mut DML_TENSOR_DESC,
    pub Epsilon: f32,
}
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub MeanTensor: *mut DML_TENSOR_DESC,
    pub VarianceTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Spatial: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BATCH_NORMALIZATION_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_BINDING_DESC {
    pub Type: DML_BINDING_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_BINDING_DESC {}
impl ::core::clone::Clone for DML_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_BINDING_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BINDING_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_BINDING_DESC {}
impl ::core::default::Default for DML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_BINDING_PROPERTIES {
    pub RequiredDescriptorCount: u32,
    pub TemporaryResourceSize: u64,
    pub PersistentResourceSize: u64,
}
impl ::core::marker::Copy for DML_BINDING_PROPERTIES {}
impl ::core::clone::Clone for DML_BINDING_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_BINDING_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BINDING_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_BINDING_PROPERTIES {}
impl ::core::default::Default for DML_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BINDING_TABLE_DESC {
    pub Dispatchable: ::core::option::Option<IDMLDispatchable>,
    pub CPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    pub GPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    pub SizeInDescriptors: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BINDING_TABLE_DESC {
    fn clone(&self) -> Self {
        Self {
            Dispatchable: self.Dispatchable.clone(),
            CPUDescriptorHandle: self.CPUDescriptorHandle,
            GPUDescriptorHandle: self.GPUDescriptorHandle,
            SizeInDescriptors: self.SizeInDescriptors,
        }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::core::Abi for DML_BINDING_TABLE_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BINDING_TABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Dispatchable == other.Dispatchable && self.CPUDescriptorHandle == other.CPUDescriptorHandle && self.GPUDescriptorHandle == other.GPUDescriptorHandle && self.SizeInDescriptors == other.SizeInDescriptors
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BINDING_TABLE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BINDING_TABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_BINDING_TYPE = i32;
pub const DML_BINDING_TYPE_NONE: DML_BINDING_TYPE = 0i32;
pub const DML_BINDING_TYPE_BUFFER: DML_BINDING_TYPE = 1i32;
pub const DML_BINDING_TYPE_BUFFER_ARRAY: DML_BINDING_TYPE = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_ARRAY_BINDING {
    pub BindingCount: u32,
    pub Bindings: *mut DML_BUFFER_BINDING,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_ARRAY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::core::Abi for DML_BUFFER_ARRAY_BINDING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_ARRAY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BUFFER_ARRAY_BINDING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_ARRAY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_BINDING {
    pub Buffer: ::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    pub Offset: u64,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_BINDING {
    fn clone(&self) -> Self {
        Self { Buffer: self.Buffer.clone(), Offset: self.Offset, SizeInBytes: self.SizeInBytes }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
unsafe impl ::windows::core::Abi for DML_BUFFER_BINDING {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Offset == other.Offset && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_BUFFER_TENSOR_DESC {
    pub DataType: DML_TENSOR_DATA_TYPE,
    pub Flags: DML_TENSOR_FLAGS,
    pub DimensionCount: u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
    pub TotalTensorSizeInBytes: u64,
    pub GuaranteedBaseOffsetAlignment: u32,
}
impl ::core::marker::Copy for DML_BUFFER_TENSOR_DESC {}
impl ::core::clone::Clone for DML_BUFFER_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_BUFFER_TENSOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_BUFFER_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_BUFFER_TENSOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_BUFFER_TENSOR_DESC {}
impl ::core::default::Default for DML_BUFFER_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_CAST_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_CAST_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CAST_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_CAST_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_CAST_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_CAST_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_CAST_OPERATOR_DESC {}
impl ::core::default::Default for DML_CAST_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_CONVOLUTION_DIRECTION = i32;
pub const DML_CONVOLUTION_DIRECTION_FORWARD: DML_CONVOLUTION_DIRECTION = 0i32;
pub const DML_CONVOLUTION_DIRECTION_BACKWARD: DML_CONVOLUTION_DIRECTION = 1i32;
#[repr(C)]
pub struct DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub GroupCount: u32,
}
impl ::core::marker::Copy for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_CONVOLUTION_INTEGER_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_CONVOLUTION_MODE = i32;
pub const DML_CONVOLUTION_MODE_CONVOLUTION: DML_CONVOLUTION_MODE = 0i32;
pub const DML_CONVOLUTION_MODE_CROSS_CORRELATION: DML_CONVOLUTION_MODE = 1i32;
#[repr(C)]
pub struct DML_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Mode: DML_CONVOLUTION_MODE,
    pub Direction: DML_CONVOLUTION_DIRECTION,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub OutputPadding: *mut u32,
    pub GroupCount: u32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_CONVOLUTION_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_CONVOLUTION_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_CREATE_DEVICE_FLAGS = u32;
pub const DML_CREATE_DEVICE_FLAG_NONE: DML_CREATE_DEVICE_FLAGS = 0u32;
pub const DML_CREATE_DEVICE_FLAG_DEBUG: DML_CREATE_DEVICE_FLAGS = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveProduct: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_CUMULATIVE_PRODUCT_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveSum: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_CUMULATIVE_SUMMATION_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_DEPTH_SPACE_ORDER = i32;
pub const DML_DEPTH_SPACE_ORDER_DEPTH_COLUMN_ROW: DML_DEPTH_SPACE_ORDER = 0i32;
pub const DML_DEPTH_SPACE_ORDER_COLUMN_ROW_DEPTH: DML_DEPTH_SPACE_ORDER = 1i32;
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_DEPTH_TO_SPACE1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_DEPTH_TO_SPACE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Offset: i32,
    pub Value: f32,
}
impl ::core::marker::Copy for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_DIAGONAL_MATRIX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::default::Default for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ABS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ACOS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ADD1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ADD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ASINH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ASIN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ATANH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ATAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_CEIL_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_CLIP_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Exponent: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_COSH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_COS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ERF_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_EXP_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    pub ConditionTensor: *mut DML_TENSOR_DESC,
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_IF_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InfinityMode: DML_IS_INFINITY_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_LOG_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MAX_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MEAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MIN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ExponentTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_POW_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AScaleTensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BScaleTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub ZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_RECIP_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RoundingMode: DML_ROUNDING_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_ROUND_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_SIGN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_SINH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_SIN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_SQRT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_TANH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_TAN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleBias: *mut DML_SCALE_BIAS,
    pub Min: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_EXECUTION_FLAGS = u32;
pub const DML_EXECUTION_FLAG_NONE: DML_EXECUTION_FLAGS = 0u32;
pub const DML_EXECUTION_FLAG_ALLOW_HALF_PRECISION_COMPUTATION: DML_EXECUTION_FLAGS = 1u32;
pub const DML_EXECUTION_FLAG_DISABLE_META_COMMANDS: DML_EXECUTION_FLAGS = 2u32;
pub const DML_EXECUTION_FLAG_DESCRIPTORS_VOLATILE: DML_EXECUTION_FLAGS = 4u32;
pub type DML_FEATURE = i32;
pub const DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT: DML_FEATURE = 0i32;
pub const DML_FEATURE_FEATURE_LEVELS: DML_FEATURE = 1i32;
#[repr(C)]
pub struct DML_FEATURE_DATA_FEATURE_LEVELS {
    pub MaxSupportedFeatureLevel: DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_FEATURE_DATA_FEATURE_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FEATURE_DATA_FEATURE_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    pub IsSupported: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_FEATURE_LEVEL = i32;
pub const DML_FEATURE_LEVEL_1_0: DML_FEATURE_LEVEL = 4096i32;
pub const DML_FEATURE_LEVEL_2_0: DML_FEATURE_LEVEL = 8192i32;
pub const DML_FEATURE_LEVEL_2_1: DML_FEATURE_LEVEL = 8448i32;
pub const DML_FEATURE_LEVEL_3_0: DML_FEATURE_LEVEL = 12288i32;
pub const DML_FEATURE_LEVEL_3_1: DML_FEATURE_LEVEL = 12544i32;
pub const DML_FEATURE_LEVEL_4_0: DML_FEATURE_LEVEL = 16384i32;
#[repr(C)]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS {
    pub RequestedFeatureLevelCount: u32,
    pub RequestedFeatureLevels: *mut DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_FEATURE_QUERY_FEATURE_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FEATURE_QUERY_FEATURE_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    pub DataType: DML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::default::Default for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub Value: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FILL_VALUE_CONSTANT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
impl ::core::default::Default for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub ValueStart: DML_SCALAR_UNION,
    pub ValueDelta: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
impl ::core::default::Default for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GATHER_ELEMENTS_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GATHER_ELEMENTS_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GATHER_ND1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
    pub BatchDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GATHER_ND1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GATHER_ND1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GATHER_ND1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GATHER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GATHER_ND_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GATHER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GATHER_ND_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GATHER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub IndexDimensions: u32,
}
impl ::core::marker::Copy for DML_GATHER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GATHER_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GATHER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GATHER_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GATHER_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GEMM_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub CTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub TransA: DML_MATRIX_TRANSFORM,
    pub TransB: DML_MATRIX_TRANSFORM,
    pub Alpha: f32,
    pub Beta: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_GEMM_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GEMM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GEMM_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GEMM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GEMM_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GEMM_OPERATOR_DESC {}
impl ::core::default::Default for DML_GEMM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GRAPH_DESC {
    pub InputCount: u32,
    pub OutputCount: u32,
    pub NodeCount: u32,
    pub Nodes: *mut DML_GRAPH_NODE_DESC,
    pub InputEdgeCount: u32,
    pub InputEdges: *mut DML_GRAPH_EDGE_DESC,
    pub OutputEdgeCount: u32,
    pub OutputEdges: *mut DML_GRAPH_EDGE_DESC,
    pub IntermediateEdgeCount: u32,
    pub IntermediateEdges: *mut DML_GRAPH_EDGE_DESC,
}
impl ::core::marker::Copy for DML_GRAPH_DESC {}
impl ::core::clone::Clone for DML_GRAPH_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GRAPH_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GRAPH_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GRAPH_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GRAPH_DESC {}
impl ::core::default::Default for DML_GRAPH_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_GRAPH_EDGE_DESC {
    pub Type: DML_GRAPH_EDGE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GRAPH_EDGE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GRAPH_EDGE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_GRAPH_EDGE_TYPE = i32;
pub const DML_GRAPH_EDGE_TYPE_INVALID: DML_GRAPH_EDGE_TYPE = 0i32;
pub const DML_GRAPH_EDGE_TYPE_INPUT: DML_GRAPH_EDGE_TYPE = 1i32;
pub const DML_GRAPH_EDGE_TYPE_OUTPUT: DML_GRAPH_EDGE_TYPE = 2i32;
pub const DML_GRAPH_EDGE_TYPE_INTERMEDIATE: DML_GRAPH_EDGE_TYPE = 3i32;
#[repr(C)]
pub struct DML_GRAPH_NODE_DESC {
    pub Type: DML_GRAPH_NODE_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_NODE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_GRAPH_NODE_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GRAPH_NODE_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_GRAPH_NODE_DESC {}
impl ::core::default::Default for DML_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_GRAPH_NODE_TYPE = i32;
pub const DML_GRAPH_NODE_TYPE_INVALID: DML_GRAPH_NODE_TYPE = 0i32;
pub const DML_GRAPH_NODE_TYPE_OPERATOR: DML_GRAPH_NODE_TYPE = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_GRU_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub LinearBeforeReset: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_GRU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_GRU_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_GRU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_GRU_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_GRU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_INPUT_GRAPH_EDGE_DESC {
    pub GraphInputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_INPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_INPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_INPUT_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_INPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_INPUT_GRAPH_EDGE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_INPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_INPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_INTERMEDIATE_GRAPH_EDGE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_INTERPOLATION_MODE = i32;
pub const DML_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DML_INTERPOLATION_MODE = 0i32;
pub const DML_INTERPOLATION_MODE_LINEAR: DML_INTERPOLATION_MODE = 1i32;
pub type DML_IS_INFINITY_MODE = i32;
pub const DML_IS_INFINITY_MODE_EITHER: DML_IS_INFINITY_MODE = 0i32;
pub const DML_IS_INFINITY_MODE_POSITIVE: DML_IS_INFINITY_MODE = 1i32;
pub const DML_IS_INFINITY_MODE_NEGATIVE: DML_IS_INFINITY_MODE = 2i32;
#[repr(C)]
pub struct DML_JOIN_OPERATOR_DESC {
    pub InputCount: u32,
    pub InputTensors: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_JOIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_JOIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_JOIN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_JOIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_JOIN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_JOIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_JOIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_LP_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub Epsilon: f32,
    pub P: u32,
}
impl ::core::marker::Copy for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_LP_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_LP_NORMALIZATION_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_LP_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub P: u32,
}
impl ::core::marker::Copy for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_LP_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_LP_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_LP_POOLING_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LSTM_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub CellMemInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub PeepholeTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub OutputCellSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub ClipThreshold: f32,
    pub UseClipThreshold: super::super::super::Foundation::BOOL,
    pub CoupleInputForget: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LSTM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_LSTM_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LSTM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_LSTM_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LSTM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_MATRIX_TRANSFORM = i32;
pub const DML_MATRIX_TRANSFORM_NONE: DML_MATRIX_TRANSFORM = 0i32;
pub const DML_MATRIX_TRANSFORM_TRANSPOSE: DML_MATRIX_TRANSFORM = 1i32;
#[repr(C)]
pub struct DML_MAX_POOLING1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputIndicesTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MAX_POOLING1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MAX_POOLING1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_MAX_POOLING2_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputIndicesTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub Dilations: *mut u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING2_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MAX_POOLING2_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING2_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MAX_POOLING2_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING2_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub Dilations: *mut u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MAX_POOLING_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_MAX_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub WindowSize: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MAX_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MAX_POOLING_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_MAX_UNPOOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_MAX_UNPOOLING_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MAX_UNPOOLING_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ScaleTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *mut DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[repr(C)]
pub struct DML_NONZERO_COORDINATES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCountTensor: *mut DML_TENSOR_DESC,
    pub OutputCoordinatesTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_NONZERO_COORDINATES_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::default::Default for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ONE_HOT_OPERATOR_DESC {
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub ValuesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ONE_HOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ONE_HOT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ONE_HOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ONE_HOT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ONE_HOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_OPERATOR_DESC {
    pub Type: DML_OPERATOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_DESC {}
impl ::core::default::Default for DML_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_OPERATOR_GRAPH_NODE_DESC {
    pub Operator: ::core::option::Option<IDMLOperator>,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_OPERATOR_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        Self { Operator: self.Operator.clone(), Name: self.Name }
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_OPERATOR_GRAPH_NODE_DESC {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_OPERATOR_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Operator == other.Operator && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_OPERATOR_GRAPH_NODE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_OPERATOR_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_OPERATOR_TYPE = i32;
pub const DML_OPERATOR_INVALID: DML_OPERATOR_TYPE = 0i32;
pub const DML_OPERATOR_ELEMENT_WISE_IDENTITY: DML_OPERATOR_TYPE = 1i32;
pub const DML_OPERATOR_ELEMENT_WISE_ABS: DML_OPERATOR_TYPE = 2i32;
pub const DML_OPERATOR_ELEMENT_WISE_ACOS: DML_OPERATOR_TYPE = 3i32;
pub const DML_OPERATOR_ELEMENT_WISE_ADD: DML_OPERATOR_TYPE = 4i32;
pub const DML_OPERATOR_ELEMENT_WISE_ASIN: DML_OPERATOR_TYPE = 5i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATAN: DML_OPERATOR_TYPE = 6i32;
pub const DML_OPERATOR_ELEMENT_WISE_CEIL: DML_OPERATOR_TYPE = 7i32;
pub const DML_OPERATOR_ELEMENT_WISE_CLIP: DML_OPERATOR_TYPE = 8i32;
pub const DML_OPERATOR_ELEMENT_WISE_COS: DML_OPERATOR_TYPE = 9i32;
pub const DML_OPERATOR_ELEMENT_WISE_DIVIDE: DML_OPERATOR_TYPE = 10i32;
pub const DML_OPERATOR_ELEMENT_WISE_EXP: DML_OPERATOR_TYPE = 11i32;
pub const DML_OPERATOR_ELEMENT_WISE_FLOOR: DML_OPERATOR_TYPE = 12i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOG: DML_OPERATOR_TYPE = 13i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_AND: DML_OPERATOR_TYPE = 14i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_EQUALS: DML_OPERATOR_TYPE = 15i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN: DML_OPERATOR_TYPE = 16i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN: DML_OPERATOR_TYPE = 17i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_NOT: DML_OPERATOR_TYPE = 18i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_OR: DML_OPERATOR_TYPE = 19i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_XOR: DML_OPERATOR_TYPE = 20i32;
pub const DML_OPERATOR_ELEMENT_WISE_MAX: DML_OPERATOR_TYPE = 21i32;
pub const DML_OPERATOR_ELEMENT_WISE_MEAN: DML_OPERATOR_TYPE = 22i32;
pub const DML_OPERATOR_ELEMENT_WISE_MIN: DML_OPERATOR_TYPE = 23i32;
pub const DML_OPERATOR_ELEMENT_WISE_MULTIPLY: DML_OPERATOR_TYPE = 24i32;
pub const DML_OPERATOR_ELEMENT_WISE_POW: DML_OPERATOR_TYPE = 25i32;
pub const DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW: DML_OPERATOR_TYPE = 26i32;
pub const DML_OPERATOR_ELEMENT_WISE_RECIP: DML_OPERATOR_TYPE = 27i32;
pub const DML_OPERATOR_ELEMENT_WISE_SIN: DML_OPERATOR_TYPE = 28i32;
pub const DML_OPERATOR_ELEMENT_WISE_SQRT: DML_OPERATOR_TYPE = 29i32;
pub const DML_OPERATOR_ELEMENT_WISE_SUBTRACT: DML_OPERATOR_TYPE = 30i32;
pub const DML_OPERATOR_ELEMENT_WISE_TAN: DML_OPERATOR_TYPE = 31i32;
pub const DML_OPERATOR_ELEMENT_WISE_THRESHOLD: DML_OPERATOR_TYPE = 32i32;
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = 33i32;
pub const DML_OPERATOR_ELEMENT_WISE_DEQUANTIZE_LINEAR: DML_OPERATOR_TYPE = 34i32;
pub const DML_OPERATOR_ACTIVATION_ELU: DML_OPERATOR_TYPE = 35i32;
pub const DML_OPERATOR_ACTIVATION_HARDMAX: DML_OPERATOR_TYPE = 36i32;
pub const DML_OPERATOR_ACTIVATION_HARD_SIGMOID: DML_OPERATOR_TYPE = 37i32;
pub const DML_OPERATOR_ACTIVATION_IDENTITY: DML_OPERATOR_TYPE = 38i32;
pub const DML_OPERATOR_ACTIVATION_LEAKY_RELU: DML_OPERATOR_TYPE = 39i32;
pub const DML_OPERATOR_ACTIVATION_LINEAR: DML_OPERATOR_TYPE = 40i32;
pub const DML_OPERATOR_ACTIVATION_LOG_SOFTMAX: DML_OPERATOR_TYPE = 41i32;
pub const DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU: DML_OPERATOR_TYPE = 42i32;
pub const DML_OPERATOR_ACTIVATION_PARAMETRIC_SOFTPLUS: DML_OPERATOR_TYPE = 43i32;
pub const DML_OPERATOR_ACTIVATION_RELU: DML_OPERATOR_TYPE = 44i32;
pub const DML_OPERATOR_ACTIVATION_SCALED_ELU: DML_OPERATOR_TYPE = 45i32;
pub const DML_OPERATOR_ACTIVATION_SCALED_TANH: DML_OPERATOR_TYPE = 46i32;
pub const DML_OPERATOR_ACTIVATION_SIGMOID: DML_OPERATOR_TYPE = 47i32;
pub const DML_OPERATOR_ACTIVATION_SOFTMAX: DML_OPERATOR_TYPE = 48i32;
pub const DML_OPERATOR_ACTIVATION_SOFTPLUS: DML_OPERATOR_TYPE = 49i32;
pub const DML_OPERATOR_ACTIVATION_SOFTSIGN: DML_OPERATOR_TYPE = 50i32;
pub const DML_OPERATOR_ACTIVATION_TANH: DML_OPERATOR_TYPE = 51i32;
pub const DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU: DML_OPERATOR_TYPE = 52i32;
pub const DML_OPERATOR_CONVOLUTION: DML_OPERATOR_TYPE = 53i32;
pub const DML_OPERATOR_GEMM: DML_OPERATOR_TYPE = 54i32;
pub const DML_OPERATOR_REDUCE: DML_OPERATOR_TYPE = 55i32;
pub const DML_OPERATOR_AVERAGE_POOLING: DML_OPERATOR_TYPE = 56i32;
pub const DML_OPERATOR_LP_POOLING: DML_OPERATOR_TYPE = 57i32;
pub const DML_OPERATOR_MAX_POOLING: DML_OPERATOR_TYPE = 58i32;
pub const DML_OPERATOR_ROI_POOLING: DML_OPERATOR_TYPE = 59i32;
pub const DML_OPERATOR_SLICE: DML_OPERATOR_TYPE = 60i32;
pub const DML_OPERATOR_CAST: DML_OPERATOR_TYPE = 61i32;
pub const DML_OPERATOR_SPLIT: DML_OPERATOR_TYPE = 62i32;
pub const DML_OPERATOR_JOIN: DML_OPERATOR_TYPE = 63i32;
pub const DML_OPERATOR_PADDING: DML_OPERATOR_TYPE = 64i32;
pub const DML_OPERATOR_VALUE_SCALE_2D: DML_OPERATOR_TYPE = 65i32;
pub const DML_OPERATOR_UPSAMPLE_2D: DML_OPERATOR_TYPE = 66i32;
pub const DML_OPERATOR_GATHER: DML_OPERATOR_TYPE = 67i32;
pub const DML_OPERATOR_SPACE_TO_DEPTH: DML_OPERATOR_TYPE = 68i32;
pub const DML_OPERATOR_DEPTH_TO_SPACE: DML_OPERATOR_TYPE = 69i32;
pub const DML_OPERATOR_TILE: DML_OPERATOR_TYPE = 70i32;
pub const DML_OPERATOR_TOP_K: DML_OPERATOR_TYPE = 71i32;
pub const DML_OPERATOR_BATCH_NORMALIZATION: DML_OPERATOR_TYPE = 72i32;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION: DML_OPERATOR_TYPE = 73i32;
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION: DML_OPERATOR_TYPE = 74i32;
pub const DML_OPERATOR_LP_NORMALIZATION: DML_OPERATOR_TYPE = 75i32;
pub const DML_OPERATOR_RNN: DML_OPERATOR_TYPE = 76i32;
pub const DML_OPERATOR_LSTM: DML_OPERATOR_TYPE = 77i32;
pub const DML_OPERATOR_GRU: DML_OPERATOR_TYPE = 78i32;
pub const DML_OPERATOR_ELEMENT_WISE_SIGN: DML_OPERATOR_TYPE = 79i32;
pub const DML_OPERATOR_ELEMENT_WISE_IS_NAN: DML_OPERATOR_TYPE = 80i32;
pub const DML_OPERATOR_ELEMENT_WISE_ERF: DML_OPERATOR_TYPE = 81i32;
pub const DML_OPERATOR_ELEMENT_WISE_SINH: DML_OPERATOR_TYPE = 82i32;
pub const DML_OPERATOR_ELEMENT_WISE_COSH: DML_OPERATOR_TYPE = 83i32;
pub const DML_OPERATOR_ELEMENT_WISE_TANH: DML_OPERATOR_TYPE = 84i32;
pub const DML_OPERATOR_ELEMENT_WISE_ASINH: DML_OPERATOR_TYPE = 85i32;
pub const DML_OPERATOR_ELEMENT_WISE_ACOSH: DML_OPERATOR_TYPE = 86i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATANH: DML_OPERATOR_TYPE = 87i32;
pub const DML_OPERATOR_ELEMENT_WISE_IF: DML_OPERATOR_TYPE = 88i32;
pub const DML_OPERATOR_ELEMENT_WISE_ADD1: DML_OPERATOR_TYPE = 89i32;
pub const DML_OPERATOR_ACTIVATION_SHRINK: DML_OPERATOR_TYPE = 90i32;
pub const DML_OPERATOR_MAX_POOLING1: DML_OPERATOR_TYPE = 91i32;
pub const DML_OPERATOR_MAX_UNPOOLING: DML_OPERATOR_TYPE = 92i32;
pub const DML_OPERATOR_DIAGONAL_MATRIX: DML_OPERATOR_TYPE = 93i32;
pub const DML_OPERATOR_SCATTER_ELEMENTS: DML_OPERATOR_TYPE = 94i32;
pub const DML_OPERATOR_SCATTER: DML_OPERATOR_TYPE = 94i32;
pub const DML_OPERATOR_ONE_HOT: DML_OPERATOR_TYPE = 95i32;
pub const DML_OPERATOR_RESAMPLE: DML_OPERATOR_TYPE = 96i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_LEFT: DML_OPERATOR_TYPE = 97i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_RIGHT: DML_OPERATOR_TYPE = 98i32;
pub const DML_OPERATOR_ELEMENT_WISE_ROUND: DML_OPERATOR_TYPE = 99i32;
pub const DML_OPERATOR_ELEMENT_WISE_IS_INFINITY: DML_OPERATOR_TYPE = 100i32;
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_TRUNCATE: DML_OPERATOR_TYPE = 101i32;
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_FLOOR: DML_OPERATOR_TYPE = 102i32;
pub const DML_OPERATOR_FILL_VALUE_CONSTANT: DML_OPERATOR_TYPE = 103i32;
pub const DML_OPERATOR_FILL_VALUE_SEQUENCE: DML_OPERATOR_TYPE = 104i32;
pub const DML_OPERATOR_CUMULATIVE_SUMMATION: DML_OPERATOR_TYPE = 105i32;
pub const DML_OPERATOR_REVERSE_SUBSEQUENCES: DML_OPERATOR_TYPE = 106i32;
pub const DML_OPERATOR_GATHER_ELEMENTS: DML_OPERATOR_TYPE = 107i32;
pub const DML_OPERATOR_GATHER_ND: DML_OPERATOR_TYPE = 108i32;
pub const DML_OPERATOR_SCATTER_ND: DML_OPERATOR_TYPE = 109i32;
pub const DML_OPERATOR_MAX_POOLING2: DML_OPERATOR_TYPE = 110i32;
pub const DML_OPERATOR_SLICE1: DML_OPERATOR_TYPE = 111i32;
pub const DML_OPERATOR_TOP_K1: DML_OPERATOR_TYPE = 112i32;
pub const DML_OPERATOR_DEPTH_TO_SPACE1: DML_OPERATOR_TYPE = 113i32;
pub const DML_OPERATOR_SPACE_TO_DEPTH1: DML_OPERATOR_TYPE = 114i32;
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1: DML_OPERATOR_TYPE = 115i32;
pub const DML_OPERATOR_RESAMPLE1: DML_OPERATOR_TYPE = 116i32;
pub const DML_OPERATOR_MATRIX_MULTIPLY_INTEGER: DML_OPERATOR_TYPE = 117i32;
pub const DML_OPERATOR_QUANTIZED_LINEAR_MATRIX_MULTIPLY: DML_OPERATOR_TYPE = 118i32;
pub const DML_OPERATOR_CONVOLUTION_INTEGER: DML_OPERATOR_TYPE = 119i32;
pub const DML_OPERATOR_QUANTIZED_LINEAR_CONVOLUTION: DML_OPERATOR_TYPE = 120i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_AND: DML_OPERATOR_TYPE = 121i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_OR: DML_OPERATOR_TYPE = 122i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_XOR: DML_OPERATOR_TYPE = 123i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_NOT: DML_OPERATOR_TYPE = 124i32;
pub const DML_OPERATOR_ELEMENT_WISE_BIT_COUNT: DML_OPERATOR_TYPE = 125i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL: DML_OPERATOR_TYPE = 126i32;
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL: DML_OPERATOR_TYPE = 127i32;
pub const DML_OPERATOR_ACTIVATION_CELU: DML_OPERATOR_TYPE = 128i32;
pub const DML_OPERATOR_ACTIVATION_RELU_GRAD: DML_OPERATOR_TYPE = 129i32;
pub const DML_OPERATOR_AVERAGE_POOLING_GRAD: DML_OPERATOR_TYPE = 130i32;
pub const DML_OPERATOR_MAX_POOLING_GRAD: DML_OPERATOR_TYPE = 131i32;
pub const DML_OPERATOR_RANDOM_GENERATOR: DML_OPERATOR_TYPE = 132i32;
pub const DML_OPERATOR_NONZERO_COORDINATES: DML_OPERATOR_TYPE = 133i32;
pub const DML_OPERATOR_RESAMPLE_GRAD: DML_OPERATOR_TYPE = 134i32;
pub const DML_OPERATOR_SLICE_GRAD: DML_OPERATOR_TYPE = 135i32;
pub const DML_OPERATOR_ADAM_OPTIMIZER: DML_OPERATOR_TYPE = 136i32;
pub const DML_OPERATOR_ARGMIN: DML_OPERATOR_TYPE = 137i32;
pub const DML_OPERATOR_ARGMAX: DML_OPERATOR_TYPE = 138i32;
pub const DML_OPERATOR_ROI_ALIGN: DML_OPERATOR_TYPE = 139i32;
pub const DML_OPERATOR_GATHER_ND1: DML_OPERATOR_TYPE = 140i32;
pub const DML_OPERATOR_ELEMENT_WISE_ATAN_YX: DML_OPERATOR_TYPE = 141i32;
pub const DML_OPERATOR_ELEMENT_WISE_CLIP_GRAD: DML_OPERATOR_TYPE = 142i32;
pub const DML_OPERATOR_ELEMENT_WISE_DIFFERENCE_SQUARE: DML_OPERATOR_TYPE = 143i32;
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = 144i32;
pub const DML_OPERATOR_CUMULATIVE_PRODUCT: DML_OPERATOR_TYPE = 145i32;
pub const DML_OPERATOR_BATCH_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = 146i32;
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZED_LINEAR_ADD: DML_OPERATOR_TYPE = 147i32;
pub const DML_OPERATOR_DYNAMIC_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = 148i32;
pub const DML_OPERATOR_ROI_ALIGN1: DML_OPERATOR_TYPE = 149i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub GraphOutputIndex: u32,
    pub Name: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_OUTPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_OUTPUT_GRAPH_EDGE_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_OUTPUT_GRAPH_EDGE_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_OUTPUT_GRAPH_EDGE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_PADDING_MODE = i32;
pub const DML_PADDING_MODE_CONSTANT: DML_PADDING_MODE = 0i32;
pub const DML_PADDING_MODE_EDGE: DML_PADDING_MODE = 1i32;
pub const DML_PADDING_MODE_REFLECTION: DML_PADDING_MODE = 2i32;
pub const DML_PADDING_MODE_SYMMETRIC: DML_PADDING_MODE = 3i32;
#[repr(C)]
pub struct DML_PADDING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub PaddingMode: DML_PADDING_MODE,
    pub PaddingValue: f32,
    pub DimensionCount: u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
}
impl ::core::marker::Copy for DML_PADDING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_PADDING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_PADDING_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_PADDING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_PADDING_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_PADDING_OPERATOR_DESC {}
impl ::core::default::Default for DML_PADDING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[repr(C)]
pub struct DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub InputScaleTensor: *mut DML_TENSOR_DESC,
    pub InputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub FilterTensor: *mut DML_TENSOR_DESC,
    pub FilterScaleTensor: *mut DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *mut u32,
    pub Dilations: *mut u32,
    pub StartPadding: *mut u32,
    pub EndPadding: *mut u32,
    pub GroupCount: u32,
}
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *mut DML_TENSOR_DESC,
    pub AScaleTensor: *mut DML_TENSOR_DESC,
    pub AZeroPointTensor: *mut DML_TENSOR_DESC,
    pub BTensor: *mut DML_TENSOR_DESC,
    pub BScaleTensor: *mut DML_TENSOR_DESC,
    pub BZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputScaleTensor: *mut DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_RANDOM_GENERATOR_OPERATOR_DESC {
    pub InputStateTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub OutputStateTensor: *mut DML_TENSOR_DESC,
    pub Type: DML_RANDOM_GENERATOR_TYPE,
}
impl ::core::marker::Copy for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_RANDOM_GENERATOR_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_RANDOM_GENERATOR_TYPE = i32;
pub const DML_RANDOM_GENERATOR_TYPE_PHILOX_4X32_10: DML_RANDOM_GENERATOR_TYPE = 0i32;
pub type DML_RECURRENT_NETWORK_DIRECTION = i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_FORWARD: DML_RECURRENT_NETWORK_DIRECTION = 0i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_BACKWARD: DML_RECURRENT_NETWORK_DIRECTION = 1i32;
pub const DML_RECURRENT_NETWORK_DIRECTION_BIDIRECTIONAL: DML_RECURRENT_NETWORK_DIRECTION = 2i32;
pub type DML_REDUCE_FUNCTION = i32;
pub const DML_REDUCE_FUNCTION_ARGMAX: DML_REDUCE_FUNCTION = 0i32;
pub const DML_REDUCE_FUNCTION_ARGMIN: DML_REDUCE_FUNCTION = 1i32;
pub const DML_REDUCE_FUNCTION_AVERAGE: DML_REDUCE_FUNCTION = 2i32;
pub const DML_REDUCE_FUNCTION_L1: DML_REDUCE_FUNCTION = 3i32;
pub const DML_REDUCE_FUNCTION_L2: DML_REDUCE_FUNCTION = 4i32;
pub const DML_REDUCE_FUNCTION_LOG_SUM: DML_REDUCE_FUNCTION = 5i32;
pub const DML_REDUCE_FUNCTION_LOG_SUM_EXP: DML_REDUCE_FUNCTION = 6i32;
pub const DML_REDUCE_FUNCTION_MAX: DML_REDUCE_FUNCTION = 7i32;
pub const DML_REDUCE_FUNCTION_MIN: DML_REDUCE_FUNCTION = 8i32;
pub const DML_REDUCE_FUNCTION_MULTIPLY: DML_REDUCE_FUNCTION = 9i32;
pub const DML_REDUCE_FUNCTION_SUM: DML_REDUCE_FUNCTION = 10i32;
pub const DML_REDUCE_FUNCTION_SUM_SQUARE: DML_REDUCE_FUNCTION = 11i32;
#[repr(C)]
pub struct DML_REDUCE_OPERATOR_DESC {
    pub Function: DML_REDUCE_FUNCTION,
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *mut u32,
}
impl ::core::marker::Copy for DML_REDUCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REDUCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_REDUCE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_REDUCE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_REDUCE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_REDUCE_OPERATOR_DESC {}
impl ::core::default::Default for DML_REDUCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_RESAMPLE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_RESAMPLE1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_RESAMPLE1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_RESAMPLE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *mut f32,
    pub InputPixelOffsets: *mut f32,
    pub OutputPixelOffsets: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_RESAMPLE_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_RESAMPLE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub ScaleCount: u32,
    pub Scales: *mut f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_RESAMPLE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_RESAMPLE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::default::Default for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_RNN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub WeightTensor: *mut DML_TENSOR_DESC,
    pub RecurrenceTensor: *mut DML_TENSOR_DESC,
    pub BiasTensor: *mut DML_TENSOR_DESC,
    pub HiddenInitTensor: *mut DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *mut DML_TENSOR_DESC,
    pub OutputSequenceTensor: *mut DML_TENSOR_DESC,
    pub OutputSingleTensor: *mut DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *mut DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
}
impl ::core::marker::Copy for DML_RNN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RNN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_RNN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_RNN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_RNN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_RNN_OPERATOR_DESC {}
impl ::core::default::Default for DML_RNN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_ROI_ALIGN1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub BatchIndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub InputPixelOffset: f32,
    pub OutputPixelOffset: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
    pub AlignRegionsToCorners: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DML_ROI_ALIGN1_OPERATOR_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ROI_ALIGN1_OPERATOR_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ROI_ALIGN_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub BatchIndicesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
}
impl ::core::marker::Copy for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_ALIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ROI_ALIGN_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ROI_ALIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ROI_ALIGN_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_ALIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_ROI_POOLING_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub ROITensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub SpatialScale: f32,
    pub PooledSize: DML_SIZE_2D,
}
impl ::core::marker::Copy for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_ROI_POOLING_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_ROI_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_ROI_POOLING_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DML_ROUNDING_MODE = i32;
pub const DML_ROUNDING_MODE_HALVES_TO_NEAREST_EVEN: DML_ROUNDING_MODE = 0i32;
pub const DML_ROUNDING_MODE_TOWARD_ZERO: DML_ROUNDING_MODE = 1i32;
pub const DML_ROUNDING_MODE_TOWARD_INFINITY: DML_ROUNDING_MODE = 2i32;
#[repr(C)]
pub union DML_SCALAR_UNION {
    pub Bytes: [u8; 8],
    pub Int8: i8,
    pub UInt8: u8,
    pub Int16: i16,
    pub UInt16: u16,
    pub Int32: i32,
    pub UInt32: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub Float32: f32,
    pub Float64: f64,
}
impl ::core::marker::Copy for DML_SCALAR_UNION {}
impl ::core::clone::Clone for DML_SCALAR_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SCALAR_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SCALAR_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SCALAR_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SCALAR_UNION {}
impl ::core::default::Default for DML_SCALAR_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SCALE_BIAS {
    pub Scale: f32,
    pub Bias: f32,
}
impl ::core::marker::Copy for DML_SCALE_BIAS {}
impl ::core::clone::Clone for DML_SCALE_BIAS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SCALE_BIAS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SCALE_BIAS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SCALE_BIAS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SCALE_BIAS {}
impl ::core::default::Default for DML_SCALE_BIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SCATTER_ND_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SCATTER_ND_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SCATTER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SCATTER_ND_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SCATTER_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub IndicesTensor: *mut DML_TENSOR_DESC,
    pub UpdatesTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SCATTER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SCATTER_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SCATTER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SCATTER_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SCATTER_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SIZE_2D {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for DML_SIZE_2D {}
impl ::core::clone::Clone for DML_SIZE_2D {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SIZE_2D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SIZE_2D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SIZE_2D>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SIZE_2D {}
impl ::core::default::Default for DML_SIZE_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SLICE1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl ::core::marker::Copy for DML_SLICE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SLICE1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SLICE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SLICE1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SLICE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SLICE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *mut DML_TENSOR_DESC,
    pub OutputGradientTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *mut u32,
    pub InputWindowSizes: *mut u32,
    pub InputWindowStrides: *mut i32,
}
impl ::core::marker::Copy for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SLICE_GRAD_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SLICE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SLICE_GRAD_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SLICE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Offsets: *mut u32,
    pub Sizes: *mut u32,
    pub Strides: *mut u32,
}
impl ::core::marker::Copy for DML_SLICE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SLICE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SLICE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SLICE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SLICE_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SPACE_TO_DEPTH1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SPACE_TO_DEPTH_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_SPLIT_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputCount: u32,
    pub OutputTensors: *mut DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SPLIT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPLIT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_SPLIT_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_SPLIT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_SPLIT_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_SPLIT_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPLIT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DML_TARGET_VERSION: u32 = 16384u32;
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
pub type DML_TENSOR_DATA_TYPE = i32;
pub const DML_TENSOR_DATA_TYPE_UNKNOWN: DML_TENSOR_DATA_TYPE = 0i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT32: DML_TENSOR_DATA_TYPE = 1i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT16: DML_TENSOR_DATA_TYPE = 2i32;
pub const DML_TENSOR_DATA_TYPE_UINT32: DML_TENSOR_DATA_TYPE = 3i32;
pub const DML_TENSOR_DATA_TYPE_UINT16: DML_TENSOR_DATA_TYPE = 4i32;
pub const DML_TENSOR_DATA_TYPE_UINT8: DML_TENSOR_DATA_TYPE = 5i32;
pub const DML_TENSOR_DATA_TYPE_INT32: DML_TENSOR_DATA_TYPE = 6i32;
pub const DML_TENSOR_DATA_TYPE_INT16: DML_TENSOR_DATA_TYPE = 7i32;
pub const DML_TENSOR_DATA_TYPE_INT8: DML_TENSOR_DATA_TYPE = 8i32;
pub const DML_TENSOR_DATA_TYPE_FLOAT64: DML_TENSOR_DATA_TYPE = 9i32;
pub const DML_TENSOR_DATA_TYPE_UINT64: DML_TENSOR_DATA_TYPE = 10i32;
pub const DML_TENSOR_DATA_TYPE_INT64: DML_TENSOR_DATA_TYPE = 11i32;
#[repr(C)]
pub struct DML_TENSOR_DESC {
    pub Type: DML_TENSOR_TYPE,
    pub Desc: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_TENSOR_DESC {}
impl ::core::clone::Clone for DML_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_TENSOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_TENSOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_TENSOR_DESC {}
impl ::core::default::Default for DML_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
pub type DML_TENSOR_FLAGS = u32;
pub const DML_TENSOR_FLAG_NONE: DML_TENSOR_FLAGS = 0u32;
pub const DML_TENSOR_FLAG_OWNED_BY_DML: DML_TENSOR_FLAGS = 1u32;
pub type DML_TENSOR_TYPE = i32;
pub const DML_TENSOR_TYPE_INVALID: DML_TENSOR_TYPE = 0i32;
pub const DML_TENSOR_TYPE_BUFFER: DML_TENSOR_TYPE = 1i32;
#[repr(C)]
pub struct DML_TILE_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub RepeatsCount: u32,
    pub Repeats: *mut u32,
}
impl ::core::marker::Copy for DML_TILE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TILE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_TILE_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_TILE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_TILE_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_TILE_OPERATOR_DESC {}
impl ::core::default::Default for DML_TILE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_TOP_K1_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_TOP_K1_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_TOP_K1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_TOP_K1_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_TOP_K_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputValueTensor: *mut DML_TENSOR_DESC,
    pub OutputIndexTensor: *mut DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
}
impl ::core::marker::Copy for DML_TOP_K_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_TOP_K_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_TOP_K_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_TOP_K_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_TOP_K_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_UPSAMPLE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub ScaleSize: DML_SIZE_2D,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_UPSAMPLE_2D_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_UPSAMPLE_2D_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DML_VALUE_SCALE_2D_OPERATOR_DESC {
    pub InputTensor: *mut DML_TENSOR_DESC,
    pub OutputTensor: *mut DML_TENSOR_DESC,
    pub Scale: f32,
    pub ChannelCount: u32,
    pub Bias: *mut f32,
}
impl ::core::marker::Copy for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DML_VALUE_SCALE_2D_OPERATOR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IDMLBindingTable(::windows::core::IUnknown);
impl IDMLBindingTable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn BindInputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(bindingcount), ::core::mem::transmute(bindings))
    }
    pub unsafe fn BindOutputs(&self, bindingcount: u32, bindings: *const DML_BINDING_DESC) {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(bindingcount), ::core::mem::transmute(bindings))
    }
    pub unsafe fn BindTemporaryResource(&self, binding: *const DML_BINDING_DESC) {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(binding))
    }
    pub unsafe fn BindPersistentResource(&self, binding: *const DML_BINDING_DESC) {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(binding))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc)).ok()
    }
}
impl ::core::convert::From<IDMLBindingTable> for IDMLDeviceChild {
    fn from(value: IDMLBindingTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLBindingTable> for IDMLDeviceChild {
    fn from(value: &IDMLBindingTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLBindingTable> for IDMLObject {
    fn from(value: IDMLBindingTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLBindingTable> for IDMLObject {
    fn from(value: &IDMLBindingTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLBindingTable> for ::windows::core::IUnknown {
    fn from(value: IDMLBindingTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLBindingTable> for ::windows::core::IUnknown {
    fn from(value: &IDMLBindingTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLBindingTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLBindingTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLBindingTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLBindingTable {}
unsafe impl ::windows::core::Interface for IDMLBindingTable {
    type Vtable = IDMLBindingTableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29c687dc_de74_4e3b_ab00_1168f2fc3cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLBindingTableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC),
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[repr(transparent)]
pub struct IDMLCommandRecorder(::windows::core::IUnknown);
impl IDMLCommandRecorder {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn RecordDispatch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Direct3D12::ID3D12CommandList>, Param1: ::windows::core::IntoParam<'a, IDMLDispatchable>, Param2: ::windows::core::IntoParam<'a, IDMLBindingTable>>(&self, commandlist: Param0, dispatchable: Param1, bindings: Param2) {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), commandlist.into_param().abi(), dispatchable.into_param().abi(), bindings.into_param().abi())
    }
}
impl ::core::convert::From<IDMLCommandRecorder> for IDMLDeviceChild {
    fn from(value: IDMLCommandRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for IDMLDeviceChild {
    fn from(value: &IDMLCommandRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCommandRecorder> for IDMLObject {
    fn from(value: IDMLCommandRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for IDMLObject {
    fn from(value: &IDMLCommandRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCommandRecorder> for ::windows::core::IUnknown {
    fn from(value: IDMLCommandRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCommandRecorder> for ::windows::core::IUnknown {
    fn from(value: &IDMLCommandRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLCommandRecorder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLCommandRecorder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLCommandRecorder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCommandRecorder {}
unsafe impl ::windows::core::Interface for IDMLCommandRecorder {
    type Vtable = IDMLCommandRecorderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6857a76_2e3e_4fdd_bff4_5d2ba10fb453);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCommandRecorderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandlist: ::windows::core::RawPtr, dispatchable: ::windows::core::RawPtr, bindings: ::windows::core::RawPtr),
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[repr(transparent)]
pub struct IDMLCompiledOperator(::windows::core::IUnknown);
impl IDMLCompiledOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLDispatchable {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLDispatchable {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDispatchable> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDispatchable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDispatchable> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDispatchable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLPageable {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLPageable {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLDeviceChild {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLDeviceChild {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for IDMLObject {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for IDMLObject {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLCompiledOperator> for ::windows::core::IUnknown {
    fn from(value: IDMLCompiledOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLCompiledOperator> for ::windows::core::IUnknown {
    fn from(value: &IDMLCompiledOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLCompiledOperator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLCompiledOperator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLCompiledOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCompiledOperator {}
unsafe impl ::windows::core::Interface for IDMLCompiledOperator {
    type Vtable = IDMLCompiledOperatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b15e56a_bf5c_4902_92d8_da3a650afea4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCompiledOperatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES),
);
#[repr(transparent)]
pub struct IDMLDebugDevice(::windows::core::IUnknown);
impl IDMLDebugDevice {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, mute: Param0) {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), mute.into_param().abi())
    }
}
impl ::core::convert::From<IDMLDebugDevice> for ::windows::core::IUnknown {
    fn from(value: IDMLDebugDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDebugDevice> for ::windows::core::IUnknown {
    fn from(value: &IDMLDebugDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLDebugDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLDebugDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLDebugDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLDebugDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDebugDevice {}
unsafe impl ::windows::core::Interface for IDMLDebugDevice {
    type Vtable = IDMLDebugDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d6f3ac9_394a_4ac3_92a7_390cc57a8217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDebugDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IDMLDevice(::windows::core::IUnknown);
impl IDMLDevice {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(featurequerydatasize), ::core::mem::transmute(featurequerydata), ::core::mem::transmute(featuresupportdatasize), ::core::mem::transmute(featuresupportdata)).ok()
    }
    pub unsafe fn CreateOperator<T: ::windows::core::Interface>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CompileOperator<'a, Param0: ::windows::core::IntoParam<'a, IDMLOperator>, T: ::windows::core::Interface>(&self, op: Param0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), op.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateOperatorInitializer<T: ::windows::core::Interface>(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateCommandRecorder<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateBindingTable<T: ::windows::core::Interface>(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Evict(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    pub unsafe fn MakeResident(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetParentDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDMLDevice> for IDMLObject {
    fn from(value: IDMLDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice> for IDMLObject {
    fn from(value: &IDMLDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDevice> for ::windows::core::IUnknown {
    fn from(value: IDMLDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice> for ::windows::core::IUnknown {
    fn from(value: &IDMLDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice {}
unsafe impl ::windows::core::Interface for IDMLDevice {
    type Vtable = IDMLDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dbd6437_96fd_423f_a98c_ae5e7c2a573f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDMLDevice1(::windows::core::IUnknown);
impl IDMLDevice1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(feature), ::core::mem::transmute(featurequerydatasize), ::core::mem::transmute(featurequerydata), ::core::mem::transmute(featuresupportdatasize), ::core::mem::transmute(featuresupportdata)).ok()
    }
    pub unsafe fn CreateOperator<T: ::windows::core::Interface>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CompileOperator<'a, Param0: ::windows::core::IntoParam<'a, IDMLOperator>, T: ::windows::core::Interface>(&self, op: Param0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), op.into_param().abi(), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateOperatorInitializer<T: ::windows::core::Interface>(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateCommandRecorder<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateBindingTable<T: ::windows::core::Interface>(&self, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Evict(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    pub unsafe fn MakeResident(&self, count: u32, ppobjects: *const ::core::option::Option<IDMLPageable>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(ppobjects)).ok()
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetParentDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CompileGraph<T: ::windows::core::Interface>(&self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(desc), ::core::mem::transmute(flags), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::convert::From<IDMLDevice1> for IDMLDevice {
    fn from(value: IDMLDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice1> for IDMLDevice {
    fn from(value: &IDMLDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDevice> for IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDevice> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDevice> for &IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDevice> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDevice1> for IDMLObject {
    fn from(value: IDMLDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice1> for IDMLObject {
    fn from(value: &IDMLDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDevice1> for ::windows::core::IUnknown {
    fn from(value: IDMLDevice1) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDevice1> for ::windows::core::IUnknown {
    fn from(value: &IDMLDevice1) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLDevice1 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice1 {}
unsafe impl ::windows::core::Interface for IDMLDevice1 {
    type Vtable = IDMLDevice1Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0884f9a_d2be_4355_aa5d_5901281ad1d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDevice1Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDMLDeviceChild(::windows::core::IUnknown);
impl IDMLDeviceChild {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDMLDeviceChild> for IDMLObject {
    fn from(value: IDMLDeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDeviceChild> for IDMLObject {
    fn from(value: &IDMLDeviceChild) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLDeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLDeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDeviceChild> for ::windows::core::IUnknown {
    fn from(value: IDMLDeviceChild) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDeviceChild> for ::windows::core::IUnknown {
    fn from(value: &IDMLDeviceChild) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLDeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLDeviceChild {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLDeviceChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLDeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDeviceChild {}
unsafe impl ::windows::core::Interface for IDMLDeviceChild {
    type Vtable = IDMLDeviceChildVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e83142_8165_49e3_974e_2fd66e4cb69d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDeviceChildVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDMLDispatchable(::windows::core::IUnknown);
impl IDMLDispatchable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLPageable {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLPageable {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLDeviceChild {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLDeviceChild {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDispatchable> for IDMLObject {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for IDMLObject {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLDispatchable> for ::windows::core::IUnknown {
    fn from(value: IDMLDispatchable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLDispatchable> for ::windows::core::IUnknown {
    fn from(value: &IDMLDispatchable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLDispatchable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLDispatchable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLDispatchable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDispatchable {}
unsafe impl ::windows::core::Interface for IDMLDispatchable {
    type Vtable = IDMLDispatchableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb821a8_1039_441e_9f1c_b1759c2f3cec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDispatchableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES),
);
#[repr(transparent)]
pub struct IDMLObject(::windows::core::IUnknown);
impl IDMLObject {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDMLObject> for ::windows::core::IUnknown {
    fn from(value: IDMLObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLObject> for ::windows::core::IUnknown {
    fn from(value: &IDMLObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLObject {}
unsafe impl ::windows::core::Interface for IDMLObject {
    type Vtable = IDMLObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8263aac_9e0c_4a2d_9b8e_007521a3317c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IDMLOperator(::windows::core::IUnknown);
impl IDMLOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDMLOperator> for IDMLDeviceChild {
    fn from(value: IDMLOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperator> for IDMLDeviceChild {
    fn from(value: &IDMLOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperator> for IDMLObject {
    fn from(value: IDMLOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperator> for IDMLObject {
    fn from(value: &IDMLOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperator> for ::windows::core::IUnknown {
    fn from(value: IDMLOperator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperator> for ::windows::core::IUnknown {
    fn from(value: &IDMLOperator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLOperator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLOperator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperator {}
unsafe impl ::windows::core::Interface for IDMLOperator {
    type Vtable = IDMLOperatorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26caae7a_3081_4633_9581_226fbe57695d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperatorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDMLOperatorInitializer(::windows::core::IUnknown);
impl IDMLOperatorInitializer {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    pub unsafe fn Reset(&self, operatorcount: u32, operators: *const ::core::option::Option<IDMLCompiledOperator>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(operatorcount), ::core::mem::transmute(operators)).ok()
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLDispatchable {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLDispatchable {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDispatchable> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDispatchable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDispatchable> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDispatchable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLPageable {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLPageable {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLPageable> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLPageable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLDeviceChild {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLDeviceChild {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for IDMLObject {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for IDMLObject {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLOperatorInitializer> for ::windows::core::IUnknown {
    fn from(value: IDMLOperatorInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLOperatorInitializer> for ::windows::core::IUnknown {
    fn from(value: &IDMLOperatorInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLOperatorInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLOperatorInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLOperatorInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperatorInitializer {}
unsafe impl ::windows::core::Interface for IDMLOperatorInitializer {
    type Vtable = IDMLOperatorInitializerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x427c1113_435c_469c_8676_4d5dd072f813);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperatorInitializerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDMLPageable(::windows::core::IUnknown);
impl IDMLPageable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(datasize), ::core::mem::transmute(data)).ok()
    }
    pub unsafe fn SetPrivateDataInterface<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, guid: *const ::windows::core::GUID, data: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), data.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, name: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDMLPageable> for IDMLDeviceChild {
    fn from(value: IDMLPageable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLPageable> for IDMLDeviceChild {
    fn from(value: &IDMLPageable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLDeviceChild> for &IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLDeviceChild> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLPageable> for IDMLObject {
    fn from(value: IDMLPageable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLPageable> for IDMLObject {
    fn from(value: &IDMLPageable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDMLObject> for &IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, IDMLObject> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDMLPageable> for ::windows::core::IUnknown {
    fn from(value: IDMLPageable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDMLPageable> for ::windows::core::IUnknown {
    fn from(value: &IDMLPageable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDMLPageable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDMLPageable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDMLPageable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLPageable {}
unsafe impl ::windows::core::Interface for IDMLPageable {
    type Vtable = IDMLPageableVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1ab0825_4542_4a4b_8617_6dde6e8f6201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLPageableVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
