/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `flows.netobserv.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## flows.netobserv.io/v1alpha1
- `FlowCollector`
## flows.netobserv.io/v1beta1
- `FlowCollector`
## flows.netobserv.io/v1beta2
- `FlowCollector`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
