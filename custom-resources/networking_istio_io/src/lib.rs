/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `networking.istio.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha3")]
pub mod v1alpha3;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
