/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `frrk8s.metallb.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## frrk8s.metallb.io/v1beta1
- `FRRConfiguration`
- `FRRNodeState`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
