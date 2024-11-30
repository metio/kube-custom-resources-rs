/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kuadrant_io

Custom resources in this crate belong to the `kuadrant.io` group. The following versions and custom resources are available:

## kuadrant.io/v1alpha1
- `DNSRecord`
- `ManagedZone`
## kuadrant.io/v1beta1
- `Kuadrant`
## kuadrant.io/v1beta2
- `RateLimitPolicy`
## kuadrant.io/v1beta3
- `RateLimitPolicy`
*/
pub mod v1alpha1;
pub mod v1beta1;
pub mod v1beta2;
pub mod v1beta3;
