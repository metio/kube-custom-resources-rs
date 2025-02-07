/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `metallb.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## metallb.io/v1beta1
- `BFDProfile`
- `BGPAdvertisement`
- `BGPPeer`
- `Community`
- `IPAddressPool`
- `L2Advertisement`
- `MetalLB`
- `ServiceL2Status`
## metallb.io/v1beta2
- `BGPPeer`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
