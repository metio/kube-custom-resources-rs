/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# bmc_tinkerbell_org

Custom resources in this crate belong to the `bmc.tinkerbell.org` group. The following versions and custom resources are available:

## bmc.tinkerbell.org/v1alpha1
- `Job`
- `Machine`
- `Task`
*/
pub mod v1alpha1;
