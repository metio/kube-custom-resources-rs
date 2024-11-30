/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cluster_x_k8s_io

Custom resources in this crate belong to the `cluster.x-k8s.io` group. The following versions and custom resources are available:

## cluster.x-k8s.io/v1alpha3
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
## cluster.x-k8s.io/v1alpha4
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
## cluster.x-k8s.io/v1beta1
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`
*/
pub mod v1alpha3;
pub mod v1alpha4;
pub mod v1beta1;
