/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `nfd.kubernetes.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## nfd.kubernetes.io/v1
- `NodeFeatureDiscovery`
## nfd.kubernetes.io/v1alpha1
- `NodeFeatureRule`
*/
pub mod v1;
pub mod v1alpha1;
