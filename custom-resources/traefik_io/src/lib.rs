/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `traefik.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## traefik.io/v1alpha1
- `IngressRoute`
- `IngressRouteTCP`
- `IngressRouteUDP`
- `Middleware`
- `MiddlewareTCP`
- `ServersTransport`
- `ServersTransportTCP`
- `TLSOption`
- `TLSStore`
- `TraefikService`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
