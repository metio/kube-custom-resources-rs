/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `repo-manager.pulpproject.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## repo-manager.pulpproject.org/v1beta2
- `PulpBackup`
- `PulpRestore`
- `Pulp`
*/
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
