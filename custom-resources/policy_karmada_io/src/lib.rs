/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# policy_karmada_io

Custom resources in this crate belong to the `policy.karmada.io` group. The following versions and custom resources are available:

## policy.karmada.io/v1alpha1
- `ClusterOverridePolicy`
- `ClusterPropagationPolicy`
- `FederatedResourceQuota`
- `OverridePolicy`
- `PropagationPolicy`
*/
pub mod v1alpha1;
