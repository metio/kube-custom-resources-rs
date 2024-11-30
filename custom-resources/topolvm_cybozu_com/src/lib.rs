/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# topolvm_cybozu_com

Custom resources in this crate belong to the `topolvm.cybozu.com` group. The following versions and custom resources are available:

## topolvm.cybozu.com/v1
- `LogicalVolume`
## topolvm.cybozu.com/v2
- `TopolvmCluster`
*/
pub mod v1;
pub mod v2;
