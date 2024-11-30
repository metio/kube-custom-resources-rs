/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# isindir_github_com

Custom resources in this crate belong to the `isindir.github.com` group. The following versions and custom resources are available:

## isindir.github.com/v1alpha1
- `SopsSecret`
## isindir.github.com/v1alpha2
- `SopsSecret`
## isindir.github.com/v1alpha3
- `SopsSecret`
*/
pub mod v1alpha1;
pub mod v1alpha2;
pub mod v1alpha3;
