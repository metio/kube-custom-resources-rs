/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# ceph_rook_io

Custom resources in this crate belong to the `ceph.rook.io` group. The following versions and custom resources are available:

## ceph.rook.io/v1
- `CephBlockPoolRadosNamespace`
- `CephBlockPool`
- `CephBucketNotification`
- `CephBucketTopic`
- `CephClient`
- `CephCOSIDriver`
- `CephFilesystemMirror`
- `CephFilesystem`
- `CephFilesystemSubVolumeGroup`
- `CephNFS`
- `CephObjectRealm`
- `CephObjectStore`
- `CephObjectStoreUser`
- `CephObjectZoneGroup`
- `CephObjectZone`
- `CephRBDMirror`
*/
pub mod v1;
