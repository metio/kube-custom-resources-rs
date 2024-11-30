/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kyverno_io

Custom resources in this crate belong to the `kyverno.io` group. The following versions and custom resources are available:

## kyverno.io/v1
- `ClusterPolicy`
- `Policy`
## kyverno.io/v1alpha2
- `AdmissionReport`
- `BackgroundScanReport`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`
## kyverno.io/v1beta1
- `UpdateRequest`
## kyverno.io/v2
- `AdmissionReport`
- `BackgroundScanReport`
- `CleanupPolicy`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`
- `ClusterCleanupPolicy`
- `PolicyException`
- `UpdateRequest`
## kyverno.io/v2alpha1
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `GlobalContextEntry`
- `PolicyException`
## kyverno.io/v2beta1
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `ClusterPolicy`
- `Policy`
- `PolicyException`
*/
pub mod v1;
pub mod v1alpha2;
pub mod v1beta1;
pub mod v2;
pub mod v2alpha1;
pub mod v2beta1;
