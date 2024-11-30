/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# infinispan_org

Custom resources in this crate belong to the `infinispan.org` group. The following versions and custom resources are available:

## infinispan.org/v1
- `Infinispan`
## infinispan.org/v2alpha1
- `Backup`
- `Batch`
- `Cache`
- `Restore`
*/
pub mod v1;
pub mod v2alpha1;
