/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# workload_codeflare_dev

Custom resources in this crate belong to the `workload.codeflare.dev` group. The following versions and custom resources are available:

## workload.codeflare.dev/v1beta1
- `AppWrapper`
- `SchedulingSpec`
## workload.codeflare.dev/v1beta2
- `AppWrapper`
*/
pub mod v1beta1;
pub mod v1beta2;
