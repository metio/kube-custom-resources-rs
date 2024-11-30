/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# flagger_app

Custom resources in this crate belong to the `flagger.app` group. The following versions and custom resources are available:

## flagger.app/v1beta1
- `AlertProvider`
- `Canary`
- `MetricTemplate`
*/
pub mod v1beta1;
