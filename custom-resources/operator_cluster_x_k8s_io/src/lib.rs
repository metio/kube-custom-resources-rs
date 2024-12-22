/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `operator.cluster.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## operator.cluster.x-k8s.io/v1alpha1
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`
## operator.cluster.x-k8s.io/v1alpha2
- `AddonProvider`
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
