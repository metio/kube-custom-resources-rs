/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `gloo.solo.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## gloo.solo.io/v1
- `Proxy`
- `Settings`
- `UpstreamGroup`
- `Upstream`
*/
#[cfg(feature = "v1")]
pub mod v1;
