/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# metal3_io

Custom resources in this crate belong to the `metal3.io` group. The following versions and custom resources are available:

## metal3.io/v1alpha1
- `BMCEventSubscription`
- `DataImage`
- `FirmwareSchema`
- `HardwareData`
- `HostFirmwareComponents`
- `HostFirmwareSettings`
- `PreprovisioningImage`
*/
pub mod v1alpha1;
