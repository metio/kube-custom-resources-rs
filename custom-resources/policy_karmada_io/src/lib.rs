/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `policy.karmada.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## policy.karmada.io/v1alpha1
- `ClusterOverridePolicy`
- `ClusterPropagationPolicy`
- `FederatedResourceQuota`
- `OverridePolicy`
- `PropagationPolicy`
*/
pub mod v1alpha1;
