/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# fluentbit_fluent_io

Custom resources in this crate belong to the `fluentbit.fluent.io` group. The following versions and custom resources are available:

## fluentbit.fluent.io/v1alpha2
- `ClusterFilter`
- `ClusterFluentBitConfig`
- `ClusterInput`
- `ClusterOutput`
- `ClusterParser`
- `Collector`
- `Filter`
- `FluentBitConfig`
- `FluentBit`
- `Output`
- `Parser`
*/
pub mod v1alpha2;
