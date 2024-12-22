/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `snapshot.storage.k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## snapshot.storage.k8s.io/v1
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`
## snapshot.storage.k8s.io/v1beta1
- `VolumeSnapshotClass`
- `VolumeSnapshotContent`
- `VolumeSnapshot`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
