/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `cilium.angeloxx.ch` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## cilium.angeloxx.ch/v2
- `HAEgressGatewayPolicy`
*/
#[cfg(feature = "v2")]
pub mod v2;
