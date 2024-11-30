/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# data_fluid_io

Custom resources in this crate belong to the `data.fluid.io` group. The following versions and custom resources are available:

## data.fluid.io/v1alpha1
- `AlluxioRuntime`
- `DataBackup`
- `DataLoad`
- `Dataset`
- `GooseFSRuntime`
- `JindoRuntime`
- `JuiceFSRuntime`
- `ThinRuntimeProfile`
- `ThinRuntime`
*/
pub mod v1alpha1;
