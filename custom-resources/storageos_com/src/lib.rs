/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# storageos_com

Custom resources in this crate belong to the `storageos.com` group. The following versions and custom resources are available:

## storageos.com/v1
- `StorageOSCluster`
*/
pub mod v1;
