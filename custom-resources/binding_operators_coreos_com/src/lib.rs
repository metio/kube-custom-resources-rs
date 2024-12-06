/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `binding.operators.coreos.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## binding.operators.coreos.com/v1alpha1
- `BindableKinds`
- `ServiceBinding`
*/
pub mod v1alpha1;
