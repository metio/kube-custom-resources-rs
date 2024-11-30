/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_tekton_dev

Custom resources in this crate belong to the `operator.tekton.dev` group. The following versions and custom resources are available:

## operator.tekton.dev/v1alpha1
- `TektonChain`
- `TektonConfig`
- `TektonHub`
- `TektonInstallerSet`
- `TektonPipeline`
- `TektonTrigger`
*/
pub mod v1alpha1;
