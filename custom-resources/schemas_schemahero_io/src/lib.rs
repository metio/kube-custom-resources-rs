/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `schemas.schemahero.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## schemas.schemahero.io/v1alpha4
- `DataType`
- `Migration`
- `Table`
*/
#[cfg(feature = "v1alpha4")]
pub mod v1alpha4;
