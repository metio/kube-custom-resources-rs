/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# clustertemplate_openshift_io

Custom resources in this crate belong to the `clustertemplate.openshift.io` group. The following versions and custom resources are available:

## clustertemplate.openshift.io/v1alpha1
- `ClusterTemplateInstance`
- `ClusterTemplateQuota`
- `ClusterTemplate`
- `ClusterTemplateSetup`
- `Config`
*/
pub mod v1alpha1;
