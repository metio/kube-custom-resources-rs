/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `workload.codeflare.dev` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## workload.codeflare.dev/v1beta1
- `AppWrapper`
- `SchedulingSpec`
## workload.codeflare.dev/v1beta2
- `AppWrapper`
*/
pub mod v1beta1;
pub mod v1beta2;
