<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# chaos-mesh.org

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `chaos-mesh.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### chaos-mesh.org/v1alpha1
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
