/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `apps.kubeblocks.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## apps.kubeblocks.io/v1
- `ClusterDefinition`
- `Cluster`
- `ComponentDefinition`
- `Component`
- `ComponentVersion`
- `ServiceDescriptor`
## apps.kubeblocks.io/v1alpha1
- `BackupPolicyTemplate`
- `ClusterDefinition`
- `Cluster`
- `ClusterVersion`
- `ComponentClassDefinition`
- `ComponentDefinition`
- `Component`
- `ComponentVersion`
- `ConfigConstraint`
- `Configuration`
- `OpsDefinition`
- `OpsRequest`
- `ServiceDescriptor`
## apps.kubeblocks.io/v1beta1
- `ConfigConstraint`
*/
pub mod v1;
pub mod v1alpha1;
pub mod v1beta1;
