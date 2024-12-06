/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `org.eclipse.che` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## org.eclipse.che/v1
- `CheCluster`
## org.eclipse.che/v2
- `CheCluster`
*/
pub mod v1;
pub mod v2;
