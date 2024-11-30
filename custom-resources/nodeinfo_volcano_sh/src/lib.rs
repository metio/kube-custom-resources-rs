/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# nodeinfo_volcano_sh

Custom resources in this crate belong to the `nodeinfo.volcano.sh` group. The following versions and custom resources are available:

## nodeinfo.volcano.sh/v1alpha1
- `Numatopology`
*/
pub mod v1alpha1;
