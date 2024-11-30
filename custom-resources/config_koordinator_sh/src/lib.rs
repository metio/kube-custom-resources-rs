/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# config_koordinator_sh

Custom resources in this crate belong to the `config.koordinator.sh` group. The following versions and custom resources are available:

## config.koordinator.sh/v1alpha1
- `ClusterColocationProfile`
*/
pub mod v1alpha1;
