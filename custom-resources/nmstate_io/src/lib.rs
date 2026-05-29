/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `nmstate.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## nmstate.io/v1
- `NMState`
- `NodeNetworkConfigurationPolicy`
## nmstate.io/v1beta1
- `NMState`
- `NodeNetworkConfigurationEnactment`
- `NodeNetworkConfigurationPolicy`
- `NodeNetworkState`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
