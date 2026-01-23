/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `ceph.rook.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## ceph.rook.io/v1
- `CephBlockPoolRadosNamespace`
- `CephBlockPool`
- `CephBucketNotification`
- `CephBucketTopic`
- `CephClient`
- `CephCluster`
- `CephCOSIDriver`
- `CephFilesystemMirror`
- `CephFilesystem`
- `CephFilesystemSubVolumeGroup`
- `CephNFS`
- `CephNVMeOFGateway`
- `CephObjectRealm`
- `CephObjectStore`
- `CephObjectStoreUser`
- `CephObjectZoneGroup`
- `CephObjectZone`
- `CephRBDMirror`
*/
#[cfg(feature = "v1")]
pub mod v1;
