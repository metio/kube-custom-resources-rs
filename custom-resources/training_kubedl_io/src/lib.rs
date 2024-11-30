/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# training_kubedl_io

Custom resources in this crate belong to the `training.kubedl.io` group. The following versions and custom resources are available:

## training.kubedl.io/v1alpha1
- `ElasticDLJob`
- `MarsJob`
- `MPIJob`
- `PyTorchJob`
- `TFJob`
- `XDLJob`
- `XGBoostJob`
*/
pub mod v1alpha1;
