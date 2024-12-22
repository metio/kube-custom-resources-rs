/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `app.terraform.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## app.terraform.io/v1alpha2
- `AgentPool`
- `Module`
- `Workspace`
*/
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
