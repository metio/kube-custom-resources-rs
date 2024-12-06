/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `s3.snappcloud.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## s3.snappcloud.io/v1alpha1
- `S3Bucket`
- `S3UserClaim`
- `S3User`
*/
pub mod v1alpha1;
