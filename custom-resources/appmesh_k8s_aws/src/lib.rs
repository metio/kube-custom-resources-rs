/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `appmesh.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## appmesh.k8s.aws/v1beta2
- `BackendGroup`
- `GatewayRoute`
- `Mesh`
- `VirtualGateway`
- `VirtualNode`
- `VirtualRouter`
- `VirtualService`
*/
pub mod v1beta2;
