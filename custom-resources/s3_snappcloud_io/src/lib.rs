/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# s3_snappcloud_io

Custom resources in this crate belong to the `s3.snappcloud.io` group. The following versions and custom resources are available:

## s3.snappcloud.io/v1alpha1
- `S3Bucket`
- `S3UserClaim`
- `S3User`
*/
pub mod v1alpha1;
