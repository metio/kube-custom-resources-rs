/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `trace.kubeblocks.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## trace.kubeblocks.io/v1
- `ReconciliationTrace`
*/
#[cfg(feature = "v1")]
pub mod v1;
