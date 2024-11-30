/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# chaos_mesh_org

Custom resources in this crate belong to the `chaos-mesh.org` group. The following versions and custom resources are available:

## chaos-mesh.org/v1alpha1
- `AWSChaos`
- `AzureChaos`
- `BlockChaos`
- `DNSChaos`
- `GCPChaos`
- `HTTPChaos`
- `IOChaos`
- `JVMChaos`
- `KernelChaos`
- `NetworkChaos`
- `PhysicalMachineChaos`
- `PhysicalMachine`
- `PodChaos`
- `PodHttpChaos`
- `PodIOChaos`
- `PodNetworkChaos`
- `RemoteCluster`
- `Schedule`
- `StatusCheck`
- `StressChaos`
- `TimeChaos`
- `WorkflowNode`
- `Workflow`
*/
pub mod v1alpha1;
