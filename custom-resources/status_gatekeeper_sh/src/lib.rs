/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `status.gatekeeper.sh` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## status.gatekeeper.sh/v1beta1
- `ConstraintPodStatus`
- `ConstraintTemplatePodStatus`
- `ExpansionTemplatePodStatus`
- `MutatorPodStatus`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
