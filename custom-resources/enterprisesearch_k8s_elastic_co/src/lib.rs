/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# enterprisesearch_k8s_elastic_co

Custom resources in this crate belong to the `enterprisesearch.k8s.elastic.co` group. The following versions and custom resources are available:

## enterprisesearch.k8s.elastic.co/v1
- `EnterpriseSearch`
## enterprisesearch.k8s.elastic.co/v1beta1
- `EnterpriseSearch`
*/
pub mod v1;
pub mod v1beta1;
