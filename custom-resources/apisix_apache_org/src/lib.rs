/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# apisix_apache_org

Custom resources in this crate belong to the `apisix.apache.org` group. The following versions and custom resources are available:

## apisix.apache.org/v2
- `ApisixClusterConfig`
- `ApisixConsumer`
- `ApisixGlobalRule`
- `ApisixPluginConfig`
- `ApisixRoute`
- `ApisixTls`
- `ApisixUpstream`
*/
pub mod v2;
