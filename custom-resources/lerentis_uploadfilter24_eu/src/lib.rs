/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `lerentis.uploadfilter24.eu` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## lerentis.uploadfilter24.eu/v1beta4
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`
## lerentis.uploadfilter24.eu/v1beta5
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`
## lerentis.uploadfilter24.eu/v1beta6
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`
## lerentis.uploadfilter24.eu/v1beta7
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`
## lerentis.uploadfilter24.eu/v1beta8
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`
*/
#[cfg(feature = "v1beta4")]
pub mod v1beta4;
#[cfg(feature = "v1beta5")]
pub mod v1beta5;
#[cfg(feature = "v1beta6")]
pub mod v1beta6;
#[cfg(feature = "v1beta7")]
pub mod v1beta7;
#[cfg(feature = "v1beta8")]
pub mod v1beta8;
