/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `tinkerbell.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## tinkerbell.org/v1alpha1
- `Stack`
- `Hardware`
- `OSIE`
- `Template`
- `Workflow`
## tinkerbell.org/v1alpha2
- `Hardware`
- `OSIE`
- `Template`
- `Workflow`
*/
pub mod v1alpha1;
pub mod v1alpha2;
