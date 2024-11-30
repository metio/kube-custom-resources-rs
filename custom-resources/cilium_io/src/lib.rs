/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cilium_io

Custom resources in this crate belong to the `cilium.io` group. The following versions and custom resources are available:

## cilium.io/v2
- `CiliumClusterwideNetworkPolicy`
- `CiliumEgressGatewayPolicy`
- `CiliumEndpoint`
- `CiliumExternalWorkload`
- `CiliumIdentity`
- `CiliumLocalRedirectPolicy`
- `CiliumNetworkPolicy`
- `CiliumNode`
## cilium.io/v2alpha1
- `CiliumBGPPeeringPolicy`
- `CiliumCIDRGroup`
- `CiliumEndpointSlice`
- `CiliumL2AnnouncementPolicy`
- `CiliumLoadBalancerIPPool`
- `CiliumNodeConfig`
- `CiliumPodIPPool`
*/
pub mod v2;
pub mod v2alpha1;
