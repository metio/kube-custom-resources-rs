/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# devices_kubeedge_io

Custom resources in this crate belong to the `devices.kubeedge.io` group. The following versions and custom resources are available:

## devices.kubeedge.io/v1alpha2
- `DeviceModel`
- `Device`
## devices.kubeedge.io/v1beta1
- `DeviceModel`
- `Device`
*/
pub mod v1alpha2;
pub mod v1beta1;
