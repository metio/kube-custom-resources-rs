/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# iot_eclipse_org

Custom resources in this crate belong to the `iot.eclipse.org` group. The following versions and custom resources are available:

## iot.eclipse.org/v1alpha1
- `Ditto`
- `Hawkbit`
*/
pub mod v1alpha1;
