/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `operator.tigera.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## operator.tigera.io/v1
- `AmazonCloudIntegration`
- `APIServer`
- `ApplicationLayer`
- `Authentication`
- `Compliance`
- `EgressGateway`
- `GatewayAPI`
- `Goldmane`
- `ImageSet`
- `Installation`
- `IntrusionDetection`
- `LogCollector`
- `LogStorage`
- `ManagementClusterConnection`
- `ManagementCluster`
- `Manager`
- `Monitor`
- `NonClusterHost`
- `PacketCaptureAPI`
- `PacketCapture`
- `PolicyRecommendation`
- `Tenant`
- `TigeraStatus`
- `TLSPassThroughRoute`
- `TLSTerminatedRoute`
- `Whisker`
## operator.tigera.io/v1beta1
- `AmazonCloudIntegration`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
