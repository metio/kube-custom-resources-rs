/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# repo_manager_pulpproject_org

Custom resources in this crate belong to the `repo-manager.pulpproject.org` group. The following versions and custom resources are available:

## repo-manager.pulpproject.org/v1beta2
- `PulpBackup`
- `PulpRestore`
- `Pulp`
*/
pub mod v1beta2;
