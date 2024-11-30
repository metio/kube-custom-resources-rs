/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# k8s_otterize_com

Custom resources in this crate belong to the `k8s.otterize.com` group. The following versions and custom resources are available:

## k8s.otterize.com/v1alpha2
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`
## k8s.otterize.com/v1alpha3
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`
*/
pub mod v1alpha2;
pub mod v1alpha3;
