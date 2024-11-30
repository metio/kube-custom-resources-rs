/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# getambassador_io

Custom resources in this crate belong to the `getambassador.io` group. The following versions and custom resources are available:

## getambassador.io/v3alpha1
- `AuthService`
- `ConsulResolver`
- `DevPortal`
- `Host`
- `KubernetesEndpointResolver`
- `KubernetesServiceResolver`
- `Listener`
- `LogService`
- `Module`
- `RateLimitService`
- `TCPMapping`
- `TLSContext`
- `TracingService`
*/
pub mod v3alpha1;
