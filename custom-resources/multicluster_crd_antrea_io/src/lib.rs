/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `multicluster.crd.antrea.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## multicluster.crd.antrea.io/v1alpha1
- `ClusterInfoImport`
- `ClusterSet`
- `Gateway`
- `LabelIdentity`
- `MemberClusterAnnounce`
- `MultiClusterConfig`
- `ResourceExport`
- `ResourceImport`
## multicluster.crd.antrea.io/v1alpha2
- `ClusterClaim`
- `ClusterSet`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
