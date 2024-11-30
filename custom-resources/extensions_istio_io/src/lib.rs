/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# extensions_istio_io

Custom resources in this crate belong to the `extensions.istio.io` group. The following versions and custom resources are available:

## extensions.istio.io/v1alpha1
- `WasmPlugin`
*/
pub mod v1alpha1;
