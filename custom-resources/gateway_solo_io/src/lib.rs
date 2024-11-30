/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# gateway_solo_io

Custom resources in this crate belong to the `gateway.solo.io` group. The following versions and custom resources are available:

## gateway.solo.io/v1
- `Gateway`
- `MatchableHttpGateway`
- `RouteOption`
- `RouteTable`
- `VirtualHostOption`
- `VirtualService`
*/
pub mod v1;
