/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `security.istio.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## security.istio.io/v1
- `AuthorizationPolicy`
- `PeerAuthentication`
- `RequestAuthentication`
## security.istio.io/v1beta1
- `AuthorizationPolicy`
- `PeerAuthentication`
- `RequestAuthentication`
*/
pub mod v1;
pub mod v1beta1;
