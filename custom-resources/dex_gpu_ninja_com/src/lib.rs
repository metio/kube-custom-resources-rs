/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# dex_gpu_ninja_com

Custom resources in this crate belong to the `dex.gpu-ninja.com` group. The following versions and custom resources are available:

## dex.gpu-ninja.com/v1alpha1
- `DexIdentityProvider`
- `DexOAuth2Client`
- `DexUser`
*/
pub mod v1alpha1;
