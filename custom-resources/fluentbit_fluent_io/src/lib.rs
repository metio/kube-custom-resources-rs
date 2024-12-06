/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `fluentbit.fluent.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
