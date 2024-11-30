/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# addons_cluster_x_k8s_io

Custom resources in this crate belong to the `addons.cluster.x-k8s.io` group. The following versions and custom resources are available:

## addons.cluster.x-k8s.io/v1alpha3
- `ClusterResourceSetBinding`
- `ClusterResourceSet`
## addons.cluster.x-k8s.io/v1alpha4
- `ClusterResourceSetBinding`
- `ClusterResourceSet`
## addons.cluster.x-k8s.io/v1beta1
- `ClusterResourceSetBinding`
- `ClusterResourceSet`
*/
pub mod v1alpha3;
pub mod v1alpha4;
pub mod v1beta1;
