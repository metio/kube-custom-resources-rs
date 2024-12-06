/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `core.kubeadmiral.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
