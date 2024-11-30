/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# sparkoperator_k8s_io

Custom resources in this crate belong to the `sparkoperator.k8s.io` group. The following versions and custom resources are available:

## sparkoperator.k8s.io/v1beta2
- `ScheduledSparkApplication`
- `SparkApplication`
*/
pub mod v1beta2;
