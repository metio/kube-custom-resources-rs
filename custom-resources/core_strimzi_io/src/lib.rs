/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `core.strimzi.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## core.strimzi.io/v1beta2
- `StrimziPodSet`
*/
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
