/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kustomize.toolkit.fluxcd.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## kustomize.toolkit.fluxcd.io/v1
- `Kustomization`
## kustomize.toolkit.fluxcd.io/v1beta1
- `Kustomization`
## kustomize.toolkit.fluxcd.io/v1beta2
- `Kustomization`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
