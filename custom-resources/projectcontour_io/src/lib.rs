/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# projectcontour_io

Custom resources in this crate belong to the `projectcontour.io` group. The following versions and custom resources are available:

## projectcontour.io/v1
- `HTTPProxy`
- `TLSCertificateDelegation`
## projectcontour.io/v1alpha1
- `ContourConfiguration`
- `ContourDeployment`
- `ExtensionService`
*/
pub mod v1;
pub mod v1alpha1;
