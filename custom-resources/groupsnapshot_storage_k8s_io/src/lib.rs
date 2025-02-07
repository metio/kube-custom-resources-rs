/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `groupsnapshot.storage.k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## groupsnapshot.storage.k8s.io/v1alpha1
- `VolumeGroupSnapshotClass`
- `VolumeGroupSnapshotContent`
- `VolumeGroupSnapshot`
## groupsnapshot.storage.k8s.io/v1beta1
- `VolumeGroupSnapshotClass`
- `VolumeGroupSnapshotContent`
- `VolumeGroupSnapshot`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
