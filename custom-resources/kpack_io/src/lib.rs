/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kpack.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## kpack.io/v1alpha1
- `Builder`
- `Build`
- `ClusterBuilder`
- `ClusterStack`
- `ClusterStore`
- `Image`
- `SourceResolver`
## kpack.io/v1alpha2
- `Builder`
- `Buildpack`
- `Build`
- `ClusterBuilder`
- `ClusterBuildpack`
- `ClusterLifecycle`
- `ClusterStack`
- `ClusterStore`
- `Image`
- `SourceResolver`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
