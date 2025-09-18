/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `apiextensions.crossplane.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## apiextensions.crossplane.io/v1
- `CompositeResourceDefinition`
- `CompositionRevision`
- `Composition`
## apiextensions.crossplane.io/v1beta1
- `CompositionRevision`
## apiextensions.crossplane.io/v2
- `CompositeResourceDefinition`
## apiextensions.crossplane.io/v2alpha1
- `CompositeResourceDefinition`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v2alpha1")]
pub mod v2alpha1;
