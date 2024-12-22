/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `appprotect.f5.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## appprotect.f5.com/v1beta1
- `APLogConf`
- `APUserSig`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
