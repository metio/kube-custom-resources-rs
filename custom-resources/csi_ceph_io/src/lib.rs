/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `csi.ceph.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## csi.ceph.io/v1
- `CephConnection`
- `ClientProfileMapping`
- `ClientProfile`
- `Driver`
- `OperatorConfig`
## csi.ceph.io/v1alpha1
- `CephConnection`
- `ClientProfileMapping`
- `ClientProfile`
- `Driver`
- `OperatorConfig`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
