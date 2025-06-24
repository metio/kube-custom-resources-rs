/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `radapp.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## radapp.io/v1alpha3
- `DeploymentResource`
- `DeploymentTemplate`
- `Recipe`
*/
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
