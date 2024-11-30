/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# org_eclipse_che

Custom resources in this crate belong to the `org.eclipse.che` group. The following versions and custom resources are available:

## org.eclipse.che/v1
- `CheCluster`
## org.eclipse.che/v2
- `CheCluster`
*/
pub mod v1;
pub mod v2;
