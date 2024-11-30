/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# beat_k8s_elastic_co

Custom resources in this crate belong to the `beat.k8s.elastic.co` group. The following versions and custom resources are available:

## beat.k8s.elastic.co/v1beta1
- `Beat`
*/
pub mod v1beta1;
