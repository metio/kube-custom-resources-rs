/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# upgrade_cattle_io

Custom resources in this crate belong to the `upgrade.cattle.io` group. The following versions and custom resources are available:

## upgrade.cattle.io/v1
- `Plan`
*/
pub mod v1;
