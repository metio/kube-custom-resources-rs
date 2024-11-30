/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# secrets_store_csi_x_k8s_io

Custom resources in this crate belong to the `secrets-store.csi.x-k8s.io` group. The following versions and custom resources are available:

## secrets-store.csi.x-k8s.io/v1
- `SecretProviderClass`
- `SecretProviderClassPodStatus`
## secrets-store.csi.x-k8s.io/v1alpha1
- `SecretProviderClass`
- `SecretProviderClassPodStatus`
*/
pub mod v1;
pub mod v1alpha1;
