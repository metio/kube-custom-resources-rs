/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `secrets-store.csi.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## secrets-store.csi.x-k8s.io/v1
- `SecretProviderClass`
- `SecretProviderClassPodStatus`
## secrets-store.csi.x-k8s.io/v1alpha1
- `SecretProviderClass`
- `SecretProviderClassPodStatus`
*/
pub mod v1;
pub mod v1alpha1;
