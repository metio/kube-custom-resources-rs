/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# rc_app_stacks

Custom resources in this crate belong to the `rc.app.stacks` group. The following versions and custom resources are available:

## rc.app.stacks/v1
- `RuntimeComponent`
- `RuntimeOperation`
## rc.app.stacks/v1beta2
- `RuntimeComponent`
- `RuntimeOperation`
*/
pub mod v1;
pub mod v1beta2;
