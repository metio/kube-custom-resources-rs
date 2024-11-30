/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# ocmagent_managed_openshift_io

Custom resources in this crate belong to the `ocmagent.managed.openshift.io` group. The following versions and custom resources are available:

## ocmagent.managed.openshift.io/v1alpha1
- `ManagedFleetNotificationRecord`
- `ManagedFleetNotification`
- `ManagedNotification`
- `OcmAgent`
*/
pub mod v1alpha1;
