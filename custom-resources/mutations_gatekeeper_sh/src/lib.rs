/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# mutations_gatekeeper_sh

Custom resources in this crate belong to the `mutations.gatekeeper.sh` group. The following versions and custom resources are available:

## mutations.gatekeeper.sh/v1
- `Assign`
- `AssignMetadata`
- `ModifySet`
## mutations.gatekeeper.sh/v1alpha1
- `Assign`
- `AssignImage`
- `AssignMetadata`
- `ModifySet`
## mutations.gatekeeper.sh/v1beta1
- `Assign`
- `AssignMetadata`
- `ModifySet`
*/
pub mod v1;
pub mod v1alpha1;
pub mod v1beta1;
