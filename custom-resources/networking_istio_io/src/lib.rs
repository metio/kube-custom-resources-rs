/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# networking_istio_io

Custom resources in this crate belong to the `networking.istio.io` group. The following versions and custom resources are available:

## networking.istio.io/v1
- `DestinationRule`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
## networking.istio.io/v1alpha3
- `DestinationRule`
- `EnvoyFilter`
- `Gateway`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
## networking.istio.io/v1beta1
- `DestinationRule`
- `Gateway`
- `ProxyConfig`
- `ServiceEntry`
- `Sidecar`
- `VirtualService`
- `WorkloadEntry`
- `WorkloadGroup`
*/
pub mod v1;
pub mod v1alpha3;
pub mod v1beta1;
