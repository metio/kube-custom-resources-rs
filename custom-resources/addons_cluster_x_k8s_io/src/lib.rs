/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `addons.cluster.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
#[cfg(feature = "v1alpha4")]
pub mod v1alpha4;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
