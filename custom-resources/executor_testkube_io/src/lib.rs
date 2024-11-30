/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# executor_testkube_io

Custom resources in this crate belong to the `executor.testkube.io` group. The following versions and custom resources are available:

## executor.testkube.io/v1
- `Executor`
- `Webhook`
*/
pub mod v1;
