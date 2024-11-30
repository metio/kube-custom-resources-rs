/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# work_karmada_io

Custom resources in this crate belong to the `work.karmada.io` group. The following versions and custom resources are available:

## work.karmada.io/v1alpha1
- `ClusterResourceBinding`
- `ResourceBinding`
## work.karmada.io/v1alpha2
- `ClusterResourceBinding`
- `ResourceBinding`
*/
pub mod v1alpha1;
pub mod v1alpha2;
