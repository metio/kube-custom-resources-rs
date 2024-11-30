<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# infrastructure.cluster.x-k8s.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `infrastructure.cluster.x-k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### infrastructure.cluster.x-k8s.io/v1alpha1
- `KubevirtCluster`
- `KubevirtClusterTemplate`
- `KubevirtMachine`
- `KubevirtMachineTemplate`
### infrastructure.cluster.x-k8s.io/v1alpha3
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
### infrastructure.cluster.x-k8s.io/v1alpha4
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`
### infrastructure.cluster.x-k8s.io/v1beta1
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
### infrastructure.cluster.x-k8s.io/v1beta2
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`
