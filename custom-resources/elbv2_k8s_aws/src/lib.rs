/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `elbv2.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## elbv2.k8s.aws/v1alpha1
- `TargetGroupBinding`
## elbv2.k8s.aws/v1beta1
- `IngressClassParams`
- `TargetGroupBinding`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
