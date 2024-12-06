/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `loki.grafana.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
