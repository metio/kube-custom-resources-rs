/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# machine_deletion_remediation_medik8s_io

Custom resources in this crate belong to the `machine-deletion-remediation.medik8s.io` group. The following versions and custom resources are available:

## machine-deletion-remediation.medik8s.io/v1alpha1
- `MachineDeletionRemediation`
- `MachineDeletionRemediationTemplate`
*/
pub mod v1alpha1;
