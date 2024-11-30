/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# workloads_kubeblocks_io

Custom resources in this crate belong to the `workloads.kubeblocks.io` group. The following versions and custom resources are available:

## workloads.kubeblocks.io/v1
- `InstanceSet`
## workloads.kubeblocks.io/v1alpha1
- `InstanceSet`
- `ReplicatedStateMachine`
*/
pub mod v1;
pub mod v1alpha1;
