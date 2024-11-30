/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# m4e_krestomat_io

Custom resources in this crate belong to the `m4e.krestomat.io` group. The following versions and custom resources are available:

## m4e.krestomat.io/v1alpha1
- `Moodle`
- `Nginx`
- `Phpfpm`
- `Routine`
*/
pub mod v1alpha1;
