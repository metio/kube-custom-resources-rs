/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# secrets_doppler_com

Custom resources in this crate belong to the `secrets.doppler.com` group. The following versions and custom resources are available:

## secrets.doppler.com/v1alpha1
- `DopplerSecret`
*/
pub mod v1alpha1;
