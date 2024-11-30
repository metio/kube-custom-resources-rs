/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# config_karmada_io

Custom resources in this crate belong to the `config.karmada.io` group. The following versions and custom resources are available:

## config.karmada.io/v1alpha1
- `ResourceInterpreterCustomization`
- `ResourceInterpreterWebhookConfiguration`
*/
pub mod v1alpha1;
