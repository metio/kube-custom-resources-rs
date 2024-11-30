/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# maps_k8s_elastic_co

Custom resources in this crate belong to the `maps.k8s.elastic.co` group. The following versions and custom resources are available:

## maps.k8s.elastic.co/v1alpha1
- `ElasticMapsServer`
*/
pub mod v1alpha1;
