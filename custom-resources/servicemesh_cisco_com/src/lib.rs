/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `servicemesh.cisco.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## servicemesh.cisco.com/v1alpha1
- `IstioControlPlane`
- `IstioMesh`
- `IstioMeshGateway`
- `PeerIstioControlPlane`
*/
pub mod v1alpha1;
