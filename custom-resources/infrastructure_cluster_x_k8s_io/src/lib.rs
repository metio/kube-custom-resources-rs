/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `infrastructure.cluster.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## infrastructure.cluster.x-k8s.io/v1alpha1
- `KubevirtCluster`
- `KubevirtClusterTemplate`
- `KubevirtMachine`
- `KubevirtMachineTemplate`
## infrastructure.cluster.x-k8s.io/v1alpha3
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
## infrastructure.cluster.x-k8s.io/v1alpha4
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
## infrastructure.cluster.x-k8s.io/v1beta1
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
- `TinkerbellCluster`
- `TinkerbellMachine`
- `TinkerbellMachineTemplate`
## infrastructure.cluster.x-k8s.io/v1beta2
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
#[cfg(feature = "v1alpha4")]
pub mod v1alpha4;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
