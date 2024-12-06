/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `notification.toolkit.fluxcd.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## notification.toolkit.fluxcd.io/v1
- `Receiver`
## notification.toolkit.fluxcd.io/v1beta1
- `Alert`
- `Provider`
- `Receiver`
## notification.toolkit.fluxcd.io/v1beta2
- `Alert`
- `Provider`
- `Receiver`
## notification.toolkit.fluxcd.io/v1beta3
- `Alert`
- `Provider`
*/
pub mod v1;
pub mod v1beta1;
pub mod v1beta2;
pub mod v1beta3;
