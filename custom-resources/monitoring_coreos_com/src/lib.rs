/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# monitoring_coreos_com

Custom resources in this crate belong to the `monitoring.coreos.com` group. The following versions and custom resources are available:

## monitoring.coreos.com/v1
- `Alertmanager`
- `PodMonitor`
- `Probe`
- `Prometheus`
- `PrometheusRule`
- `ServiceMonitor`
- `ThanosRuler`
## monitoring.coreos.com/v1alpha1
- `AlertmanagerConfig`
- `PrometheusAgent`
- `ScrapeConfig`
## monitoring.coreos.com/v1beta1
- `AlertmanagerConfig`
*/
pub mod v1;
pub mod v1alpha1;
pub mod v1beta1;
