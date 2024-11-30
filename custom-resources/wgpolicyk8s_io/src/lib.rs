/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# wgpolicyk8s_io

Custom resources in this crate belong to the `wgpolicyk8s.io` group. The following versions and custom resources are available:

## wgpolicyk8s.io/v1alpha1
- `ClusterPolicyReport`
- `PolicyReport`
## wgpolicyk8s.io/v1alpha2
- `ClusterPolicyReport`
- `PolicyReport`
## wgpolicyk8s.io/v1beta1
- `ClusterPolicyReport`
- `PolicyReport`
*/
pub mod v1alpha1;
pub mod v1alpha2;
pub mod v1beta1;
