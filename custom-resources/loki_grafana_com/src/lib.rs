/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# loki_grafana_com

Custom resources in this crate belong to the `loki.grafana.com` group. The following versions and custom resources are available:

## loki.grafana.com/v1
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`
## loki.grafana.com/v1beta1
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`
*/
pub mod v1;
pub mod v1beta1;
