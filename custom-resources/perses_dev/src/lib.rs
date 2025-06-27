/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `perses.dev` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## perses.dev/v1alpha1
- `Perses`
- `PersesDashboard`
- `PersesDatasource`
## perses.dev/v1alpha2
- `Perses`
- `PersesDashboard`
- `PersesDatasource`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
