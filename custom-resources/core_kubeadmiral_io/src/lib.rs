/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# core_kubeadmiral_io

Custom resources in this crate belong to the `core.kubeadmiral.io` group. The following versions and custom resources are available:

## core.kubeadmiral.io/v1alpha1
- `ClusterCollectedStatus`
- `ClusterFederatedObject`
- `ClusterOverridePolicy`
- `ClusterPropagatedVersion`
- `ClusterPropagationPolicy`
- `CollectedStatus`
- `FederatedCluster`
- `FederatedObject`
- `FederatedTypeConfig`
- `OverridePolicy`
- `PropagatedVersion`
- `PropagationPolicy`
- `SchedulerPluginWebhookConfiguration`
- `SchedulingProfile`
*/
pub mod v1alpha1;
