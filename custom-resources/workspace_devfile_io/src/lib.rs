/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `workspace.devfile.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## workspace.devfile.io/v1alpha1
- `DevWorkspace`
- `DevWorkspaceTemplate`
## workspace.devfile.io/v1alpha2
- `DevWorkspace`
- `DevWorkspaceTemplate`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
