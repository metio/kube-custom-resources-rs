/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# keda_sh

Custom resources in this crate belong to the `keda.sh` group. The following versions and custom resources are available:

## keda.sh/v1alpha1
- `ClusterTriggerAuthentication`
- `ScaledJob`
- `ScaledObject`
- `TriggerAuthentication`
*/
pub mod v1alpha1;
