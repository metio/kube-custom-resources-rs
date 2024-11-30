/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# anywhere_eks_amazonaws_com

Custom resources in this crate belong to the `anywhere.eks.amazonaws.com` group. The following versions and custom resources are available:

## anywhere.eks.amazonaws.com/v1alpha1
- `AWSDatacenterConfig`
- `AWSIamConfig`
- `Bundles`
- `CloudStackDatacenterConfig`
- `CloudStackMachineConfig`
- `Cluster`
- `ControlPlaneUpgrade`
- `DockerDatacenterConfig`
- `EKSARelease`
- `FluxConfig`
- `GitOpsConfig`
- `MachineDeploymentUpgrade`
- `NodeUpgrade`
- `NutanixDatacenterConfig`
- `NutanixMachineConfig`
- `OIDCConfig`
- `SnowDatacenterConfig`
- `SnowIPPool`
- `SnowMachineConfig`
- `TinkerbellDatacenterConfig`
- `TinkerbellMachineConfig`
- `TinkerbellTemplateConfig`
- `VSphereDatacenterConfig`
- `VSphereMachineConfig`
*/
pub mod v1alpha1;
