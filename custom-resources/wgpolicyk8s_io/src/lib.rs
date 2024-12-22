/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `wgpolicyk8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
