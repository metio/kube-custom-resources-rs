/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `getambassador.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
