/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `bmc.tinkerbell.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## bmc.tinkerbell.org/v1alpha1
- `Job`
- `Machine`
- `Task`
*/
pub mod v1alpha1;
