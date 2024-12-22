/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `stunner.l7mp.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
