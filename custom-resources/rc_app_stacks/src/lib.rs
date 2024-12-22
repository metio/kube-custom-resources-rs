/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `rc.app.stacks` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## rc.app.stacks/v1
- `RuntimeComponent`
- `RuntimeOperation`
## rc.app.stacks/v1beta2
- `RuntimeComponent`
- `RuntimeOperation`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
