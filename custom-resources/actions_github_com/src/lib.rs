/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# actions_github_com

Custom resources in this crate belong to the `actions.github.com` group. The following versions and custom resources are available:

## actions.github.com/v1alpha1
- `AutoscalingListener`
- `AutoscalingRunnerSet`
- `EphemeralRunnerSet`
*/
pub mod v1alpha1;
