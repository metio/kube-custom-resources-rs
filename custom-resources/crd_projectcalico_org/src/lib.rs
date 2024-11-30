/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# crd_projectcalico_org

Custom resources in this crate belong to the `crd.projectcalico.org` group. The following versions and custom resources are available:

## crd.projectcalico.org/v1
- `BGPConfiguration`
- `BGPFilter`
- `BGPPeer`
- `BlockAffinity`
- `CalicoNodeStatus`
- `ClusterInformation`
- `GlobalNetworkSet`
- `HostEndpoint`
- `IPAMBlock`
- `IPAMConfig`
- `IPAMHandle`
- `IPPool`
- `IPReservation`
- `KubeControllersConfiguration`
- `NetworkSet`
- `Tier`
*/
pub mod v1;
