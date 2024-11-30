/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# rules_kubeedge_io

Custom resources in this crate belong to the `rules.kubeedge.io` group. The following versions and custom resources are available:

## rules.kubeedge.io/v1
- `RuleEndpoint`
- `Rule`
*/
pub mod v1;
