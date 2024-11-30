/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# flows_netobserv_io

Custom resources in this crate belong to the `flows.netobserv.io` group. The following versions and custom resources are available:

## flows.netobserv.io/v1alpha1
- `FlowCollector`
## flows.netobserv.io/v1beta1
- `FlowCollector`
## flows.netobserv.io/v1beta2
- `FlowCollector`
*/
pub mod v1alpha1;
pub mod v1beta1;
pub mod v1beta2;
