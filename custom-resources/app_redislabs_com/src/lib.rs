/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# app_redislabs_com

Custom resources in this crate belong to the `app.redislabs.com` group. The following versions and custom resources are available:

## app.redislabs.com/v1
- `RedisEnterpriseCluster`
## app.redislabs.com/v1alpha1
- `RedisEnterpriseActiveActiveDatabase`
- `RedisEnterpriseCluster`
- `RedisEnterpriseDatabase`
- `RedisEnterpriseRemoteCluster`
*/
pub mod v1;
pub mod v1alpha1;
