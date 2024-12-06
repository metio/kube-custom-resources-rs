/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `elasticache.services.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## elasticache.services.k8s.aws/v1alpha1
- `CacheParameterGroup`
- `CacheSubnetGroup`
- `ReplicationGroup`
- `Snapshot`
- `UserGroup`
- `User`
*/
pub mod v1alpha1;
