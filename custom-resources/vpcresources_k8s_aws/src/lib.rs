/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# vpcresources_k8s_aws

Custom resources in this crate belong to the `vpcresources.k8s.aws` group. The following versions and custom resources are available:

## vpcresources.k8s.aws/v1alpha1
- `CNINode`
## vpcresources.k8s.aws/v1beta1
- `SecurityGroupPolicy`
*/
pub mod v1alpha1;
pub mod v1beta1;
