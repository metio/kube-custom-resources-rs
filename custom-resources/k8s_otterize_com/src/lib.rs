/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `k8s.otterize.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## k8s.otterize.com/v1alpha2
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`
## k8s.otterize.com/v1alpha3
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`
*/
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
