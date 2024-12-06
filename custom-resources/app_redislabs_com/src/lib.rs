/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `app.redislabs.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
