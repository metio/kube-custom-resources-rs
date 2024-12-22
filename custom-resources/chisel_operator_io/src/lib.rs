/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `chisel-operator.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## chisel-operator.io/v1
- `ExitNodeProvisioner`
- `ExitNode`
## chisel-operator.io/v2
- `ExitNode`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;
