/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# apps_gitlab_com

Custom resources in this crate belong to the `apps.gitlab.com` group. The following versions and custom resources are available:

## apps.gitlab.com/v1beta1
- `GitLab`
## apps.gitlab.com/v1beta2
- `Runner`
*/
pub mod v1beta1;
pub mod v1beta2;
