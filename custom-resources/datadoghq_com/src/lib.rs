/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# datadoghq_com

Custom resources in this crate belong to the `datadoghq.com` group. The following versions and custom resources are available:

## datadoghq.com/v1alpha1
- `DatadogAgent`
- `DatadogMetric`
- `DatadogMonitor`
- `DatadogSLO`
## datadoghq.com/v2alpha1
- `DatadogAgent`
*/
pub mod v1alpha1;
pub mod v2alpha1;
