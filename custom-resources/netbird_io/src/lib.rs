/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `netbird.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## netbird.io/v1
- `NBGroup`
- `NBPolicy`
- `NBResource`
- `NBRoutingPeer`
- `NBSetupKey`
*/
#[cfg(feature = "v1")]
pub mod v1;
