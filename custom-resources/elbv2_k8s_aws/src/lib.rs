/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# elbv2_k8s_aws

Custom resources in this crate belong to the `elbv2.k8s.aws` group. The following versions and custom resources are available:

## elbv2.k8s.aws/v1alpha1
- `TargetGroupBinding`
## elbv2.k8s.aws/v1beta1
- `IngressClassParams`
- `TargetGroupBinding`
*/
pub mod v1alpha1;
pub mod v1beta1;
