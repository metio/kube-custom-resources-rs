/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_cluster_x_k8s_io

Custom resources in this crate belong to the `operator.cluster.x-k8s.io` group. The following versions and custom resources are available:

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
pub mod v1alpha1;
pub mod v1alpha2;
