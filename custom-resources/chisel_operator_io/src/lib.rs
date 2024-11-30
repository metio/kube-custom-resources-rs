/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# chisel_operator_io

Custom resources in this crate belong to the `chisel-operator.io` group. The following versions and custom resources are available:

## chisel-operator.io/v1
- `ExitNodeProvisioner`
- `ExitNode`
## chisel-operator.io/v2
- `ExitNode`
*/
pub mod v1;
pub mod v2;
