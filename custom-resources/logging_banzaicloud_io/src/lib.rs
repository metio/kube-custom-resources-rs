/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# logging_banzaicloud_io

Custom resources in this crate belong to the `logging.banzaicloud.io` group. The following versions and custom resources are available:

## logging.banzaicloud.io/v1alpha1
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Logging`
- `Output`
## logging.banzaicloud.io/v1beta1
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Output`
- `SyslogNGClusterFlow`
- `SyslogNGClusterOutput`
- `SyslogNGFlow`
- `SyslogNGOutput`
*/
pub mod v1alpha1;
pub mod v1beta1;
