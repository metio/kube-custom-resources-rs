/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_cryostat_io

Custom resources in this crate belong to the `operator.cryostat.io` group. The following versions and custom resources are available:

## operator.cryostat.io/v1beta1
- `Cryostat`
## operator.cryostat.io/v1beta2
- `Cryostat`
*/
pub mod v1beta1;
pub mod v1beta2;
