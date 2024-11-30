/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# snapshot_storage_k8s_io

Custom resources in this crate belong to the `snapshot.storage.k8s.io` group. The following versions and custom resources are available:

## snapshot.storage.k8s.io/v1
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`
## snapshot.storage.k8s.io/v1beta1
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`
*/
pub mod v1;
pub mod v1beta1;
