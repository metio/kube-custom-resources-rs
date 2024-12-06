/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `servicebinding.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## servicebinding.io/v1alpha3
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`
## servicebinding.io/v1beta1
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`
*/
pub mod v1alpha3;
pub mod v1beta1;
