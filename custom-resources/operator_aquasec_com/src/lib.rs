/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_aquasec_com

Custom resources in this crate belong to the `operator.aquasec.com` group. The following versions and custom resources are available:

## operator.aquasec.com/v1alpha1
- `AquaCsp`
- `AquaDatabase`
- `AquaEnforcer`
- `AquaGateway`
- `AquaKubeEnforcer`
- `AquaScanner`
- `AquaServer`
*/
pub mod v1alpha1;
