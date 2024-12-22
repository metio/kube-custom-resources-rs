/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `anywhere.eks.amazonaws.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
