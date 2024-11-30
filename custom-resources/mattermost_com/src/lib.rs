/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# mattermost_com

Custom resources in this crate belong to the `mattermost.com` group. The following versions and custom resources are available:

## mattermost.com/v1alpha1
- `ClusterInstallation`
- `MattermostRestoreDB`
*/
pub mod v1alpha1;
