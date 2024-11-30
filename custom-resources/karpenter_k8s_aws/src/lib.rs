/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# karpenter_k8s_aws

Custom resources in this crate belong to the `karpenter.k8s.aws` group. The following versions and custom resources are available:

## karpenter.k8s.aws/v1
- `EC2NodeClass`
## karpenter.k8s.aws/v1beta1
- `EC2NodeClass`
*/
pub mod v1;
pub mod v1beta1;
