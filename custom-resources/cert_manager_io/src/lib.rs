/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cert_manager_io

Custom resources in this crate belong to the `cert-manager.io` group. The following versions and custom resources are available:

## cert-manager.io/v1
- `CertificateRequest`
- `Certificate`
- `ClusterIssuer`
- `Issuer`
*/
pub mod v1;
