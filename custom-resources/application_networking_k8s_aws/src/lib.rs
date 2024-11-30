/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# application_networking_k8s_aws

Custom resources in this crate belong to the `application-networking.k8s.aws` group. The following versions and custom resources are available:

## application-networking.k8s.aws/v1alpha1
- `AccessLogPolicy`
- `IAMAuthPolicy`
- `ServiceExport`
- `ServiceImport`
- `TargetGroupPolicy`
- `VpcAssociationPolicy`
*/
pub mod v1alpha1;
