/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `forklift.konveyor.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
