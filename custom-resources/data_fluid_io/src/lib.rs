/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `data.fluid.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
