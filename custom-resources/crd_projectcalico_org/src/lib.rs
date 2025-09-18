/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `crd.projectcalico.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## crd.projectcalico.org/v1
- `BGPConfiguration`
- `BGPFilter`
- `BGPPeer`
- `BlockAffinity`
- `CalicoNodeStatus`
- `ClusterInformation`
- `FelixConfiguration`
- `GlobalNetworkPolicy`
- `GlobalNetworkSet`
- `HostEndpoint`
- `IPAMBlock`
- `IPAMConfig`
- `IPAMHandle`
- `IPPool`
- `IPReservation`
- `KubeControllersConfiguration`
- `NetworkPolicy`
- `NetworkSet`
- `StagedGlobalNetworkPolicy`
- `StagedKubernetesNetworkPolicy`
- `StagedNetworkPolicy`
- `Tier`
*/
#[cfg(feature = "v1")]
pub mod v1;
