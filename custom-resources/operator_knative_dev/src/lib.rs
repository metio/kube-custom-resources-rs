/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_knative_dev

Custom resources in this crate belong to the `operator.knative.dev` group. The following versions and custom resources are available:

## operator.knative.dev/v1beta1
- `KnativeEventing`
- `KnativeServing`
*/
pub mod v1beta1;
