/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# multicluster_crd_antrea_io

Custom resources in this crate belong to the `multicluster.crd.antrea.io` group. The following versions and custom resources are available:

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
pub mod v1alpha1;
pub mod v1alpha2;
