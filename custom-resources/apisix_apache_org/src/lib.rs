/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `apisix.apache.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## apisix.apache.org/v1alpha1
- `BackendTrafficPolicy`
- `Consumer`
- `GatewayProxy`
- `HTTPRoutePolicy`
- `PluginConfig`
## apisix.apache.org/v2
- `ApisixClusterConfig`
- `ApisixConsumer`
- `ApisixGlobalRule`
- `ApisixPluginConfig`
- `ApisixRoute`
- `ApisixTls`
- `ApisixUpstream`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v2")]
pub mod v2;
