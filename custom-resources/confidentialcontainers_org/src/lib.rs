/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `confidentialcontainers.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## confidentialcontainers.org/v1beta1
- `CcRuntime`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
