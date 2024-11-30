/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# stunner_l7mp_io

Custom resources in this crate belong to the `stunner.l7mp.io` group. The following versions and custom resources are available:

## stunner.l7mp.io/v1
- `Dataplane`
- `GatewayConfig`
- `StaticService`
- `UDPRoute`
## stunner.l7mp.io/v1alpha1
- `Dataplane`
- `GatewayConfig`
- `StaticService`
*/
pub mod v1;
pub mod v1alpha1;
