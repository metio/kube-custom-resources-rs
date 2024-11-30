<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# operator.tigera.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `operator.tigera.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### operator.tigera.io/v1
- `AmazonCloudIntegration`
- `APIServer`
- `ApplicationLayer`
- `Authentication`
- `Compliance`
- `EgressGateway`
- `ImageSet`
- `Installation`
- `IntrusionDetection`
- `LogCollector`
- `LogStorage`
- `ManagementClusterConnection`
- `ManagementCluster`
- `Manager`
- `Monitor`
- `PacketCapture`
- `PolicyRecommendation`
- `Tenant`
- `TigeraStatus`
- `TLSPassThroughRoute`
- `TLSTerminatedRoute`
### operator.tigera.io/v1beta1
- `AmazonCloudIntegration`
