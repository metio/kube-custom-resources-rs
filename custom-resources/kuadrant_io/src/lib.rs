/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kuadrant.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
