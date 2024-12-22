/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `datadoghq.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## datadoghq.com/v1alpha1
- `DatadogAgent`
- `DatadogMetric`
- `DatadogMonitor`
- `DatadogSLO`
## datadoghq.com/v2alpha1
- `DatadogAgent`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v2alpha1")]
pub mod v2alpha1;
