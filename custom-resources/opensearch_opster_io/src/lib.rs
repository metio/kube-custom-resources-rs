/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `opensearch.opster.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## opensearch.opster.io/v1
- `OpensearchActionGroup`
- `OpenSearchCluster`
- `OpensearchComponentTemplate`
- `OpensearchIndexTemplate`
- `OpenSearchISMPolicy`
- `OpensearchRole`
- `OpensearchTenant`
- `OpensearchUserRoleBinding`
- `OpensearchUser`
*/
#[cfg(feature = "v1")]
pub mod v1;
