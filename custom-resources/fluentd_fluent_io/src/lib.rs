/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `fluentd.fluent.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## fluentd.fluent.io/v1alpha1
- `ClusterFilter`
- `ClusterFluentdConfig`
- `ClusterInput`
- `ClusterOutput`
- `Filter`
- `FluentdConfig`
- `Fluentd`
- `Input`
- `Output`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
