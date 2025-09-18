/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `gateway.solo.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## gateway.solo.io/v1
- `Gateway`
- `MatchableHttpGateway`
- `RouteOption`
- `RouteTable`
- `VirtualHostOption`
- `VirtualService`
*/
#[cfg(feature = "v1")]
pub mod v1;
