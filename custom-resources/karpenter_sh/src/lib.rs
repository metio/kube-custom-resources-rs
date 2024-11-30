/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# karpenter_sh

Custom resources in this crate belong to the `karpenter.sh` group. The following versions and custom resources are available:

## karpenter.sh/v1
- `NodeClaim`
- `NodePool`
## karpenter.sh/v1beta1
- `NodeClaim`
- `NodePool`
*/
pub mod v1;
pub mod v1beta1;
