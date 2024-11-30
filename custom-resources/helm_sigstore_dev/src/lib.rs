/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# helm_sigstore_dev

Custom resources in this crate belong to the `helm.sigstore.dev` group. The following versions and custom resources are available:

## helm.sigstore.dev/v1alpha1
- `Rekor`
*/
pub mod v1alpha1;
