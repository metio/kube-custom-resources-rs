/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `apm.k8s.elastic.co` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## apm.k8s.elastic.co/v1
- `ApmServer`
## apm.k8s.elastic.co/v1beta1
- `ApmServer`
*/
pub mod v1;
pub mod v1beta1;
