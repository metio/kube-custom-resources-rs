/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `metal3.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## metal3.io/v1alpha1
- `BareMetalHost`
- `BMCEventSubscription`
- `DataImage`
- `FirmwareSchema`
- `HardwareData`
- `HostFirmwareComponents`
- `HostFirmwareSettings`
- `PreprovisioningImage`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
