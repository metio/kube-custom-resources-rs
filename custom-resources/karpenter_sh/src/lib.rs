/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `karpenter.sh` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## karpenter.sh/v1
- `NodeClaim`
- `NodePool`
## karpenter.sh/v1beta1
- `NodeClaim`
- `NodePool`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
