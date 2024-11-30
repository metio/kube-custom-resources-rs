/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# secretgenerator_mittwald_de

Custom resources in this crate belong to the `secretgenerator.mittwald.de` group. The following versions and custom resources are available:

## secretgenerator.mittwald.de/v1alpha1
- `BasicAuth`
- `SSHKeyPair`
- `StringSecret`
*/
pub mod v1alpha1;
