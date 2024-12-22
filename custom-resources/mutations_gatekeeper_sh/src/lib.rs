/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `mutations.gatekeeper.sh` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
