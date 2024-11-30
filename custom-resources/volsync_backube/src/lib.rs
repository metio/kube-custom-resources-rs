/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# volsync_backube

Custom resources in this crate belong to the `volsync.backube` group. The following versions and custom resources are available:

## volsync.backube/v1alpha1
- `ReplicationDestination`
- `ReplicationSource`
*/
pub mod v1alpha1;
