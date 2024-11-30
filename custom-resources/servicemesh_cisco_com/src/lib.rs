/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# servicemesh_cisco_com

Custom resources in this crate belong to the `servicemesh.cisco.com` group. The following versions and custom resources are available:

## servicemesh.cisco.com/v1alpha1
- `IstioControlPlane`
- `IstioMesh`
- `IstioMeshGateway`
- `PeerIstioControlPlane`
*/
pub mod v1alpha1;
