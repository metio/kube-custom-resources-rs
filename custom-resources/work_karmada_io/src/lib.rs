/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `work.karmada.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## work.karmada.io/v1alpha1
- `ClusterResourceBinding`
- `ResourceBinding`
## work.karmada.io/v1alpha2
- `ClusterResourceBinding`
- `ResourceBinding`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
