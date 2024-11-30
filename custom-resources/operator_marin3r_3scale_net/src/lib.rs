/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_marin3r_3scale_net

Custom resources in this crate belong to the `operator.marin3r.3scale.net` group. The following versions and custom resources are available:

## operator.marin3r.3scale.net/v1alpha1
- `DiscoveryServiceCertificate`
- `DiscoveryService`
- `EnvoyDeployment`
*/
pub mod v1alpha1;
