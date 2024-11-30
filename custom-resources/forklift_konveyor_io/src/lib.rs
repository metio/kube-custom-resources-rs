/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# forklift_konveyor_io

Custom resources in this crate belong to the `forklift.konveyor.io` group. The following versions and custom resources are available:

## forklift.konveyor.io/v1beta1
- `ForkliftController`
- `Hook`
- `Host`
- `Migration`
- `NetworkMap`
- `OpenstackVolumePopulator`
- `OvirtVolumePopulator`
- `Plan`
- `Provider`
- `StorageMap`
*/
pub mod v1beta1;
