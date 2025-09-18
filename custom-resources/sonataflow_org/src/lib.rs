/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `sonataflow.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## sonataflow.org/v1alpha08
- `SonataFlowBuild`
- `SonataFlowPlatform`
- `SonataFlow`
*/
#[cfg(feature = "v1alpha08")]
pub mod v1alpha08;
