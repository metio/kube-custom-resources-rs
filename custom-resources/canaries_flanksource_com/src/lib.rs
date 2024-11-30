/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# canaries_flanksource_com

Custom resources in this crate belong to the `canaries.flanksource.com` group. The following versions and custom resources are available:

## canaries.flanksource.com/v1
- `Canary`
- `Component`
- `Topology`
*/
pub mod v1;
