/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `cilium.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## cilium.io/v2
- `CiliumBGPAdvertisement`
- `CiliumBGPClusterConfig`
- `CiliumBGPNodeConfigOverride`
- `CiliumBGPNodeConfig`
- `CiliumBGPPeerConfig`
- `CiliumClusterwideNetworkPolicy`
- `CiliumEgressGatewayPolicy`
- `CiliumEndpoint`
- `CiliumExternalWorkload`
- `CiliumIdentity`
- `CiliumLocalRedirectPolicy`
- `CiliumNetworkPolicy`
- `CiliumNodeConfig`
- `CiliumNode`
## cilium.io/v2alpha1
- `CiliumBGPAdvertisement`
- `CiliumBGPClusterConfig`
- `CiliumBGPNodeConfigOverride`
- `CiliumBGPNodeConfig`
- `CiliumBGPPeerConfig`
- `CiliumBGPPeeringPolicy`
- `CiliumCIDRGroup`
- `CiliumEndpointSlice`
- `CiliumGatewayClassConfig`
- `CiliumL2AnnouncementPolicy`
- `CiliumLoadBalancerIPPool`
- `CiliumNodeConfig`
- `CiliumPodIPPool`
*/
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v2alpha1")]
pub mod v2alpha1;
