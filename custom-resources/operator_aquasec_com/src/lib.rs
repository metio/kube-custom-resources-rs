/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `operator.aquasec.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## operator.aquasec.com/v1alpha1
- `AquaCsp`
- `AquaDatabase`
- `AquaEnforcer`
- `AquaGateway`
- `AquaKubeEnforcer`
- `AquaScanner`
- `AquaServer`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
