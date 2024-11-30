/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# source_toolkit_fluxcd_io

Custom resources in this crate belong to the `source.toolkit.fluxcd.io` group. The following versions and custom resources are available:

## source.toolkit.fluxcd.io/v1
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`
## source.toolkit.fluxcd.io/v1beta1
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`
## source.toolkit.fluxcd.io/v1beta2
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`
- `OCIRepository`
*/
pub mod v1;
pub mod v1beta1;
pub mod v1beta2;
