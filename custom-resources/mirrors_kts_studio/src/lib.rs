/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# mirrors_kts_studio

Custom resources in this crate belong to the `mirrors.kts.studio` group. The following versions and custom resources are available:

## mirrors.kts.studio/v1alpha1
- `SecretMirror`
## mirrors.kts.studio/v1alpha2
- `SecretMirror`
*/
pub mod v1alpha1;
pub mod v1alpha2;
