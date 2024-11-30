/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# slo_koordinator_sh

Custom resources in this crate belong to the `slo.koordinator.sh` group. The following versions and custom resources are available:

## slo.koordinator.sh/v1alpha1
- `NodeMetric`
- `NodeSLO`
*/
pub mod v1alpha1;
