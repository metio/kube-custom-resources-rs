/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# bpfman_io

Custom resources in this crate belong to the `bpfman.io` group. The following versions and custom resources are available:

## bpfman.io/v1alpha1
- `BpfProgram`
- `FentryProgram`
- `FexitProgram`
- `KprobeProgram`
- `TcProgram`
- `TracepointProgram`
- `UprobeProgram`
- `XdpProgram`
*/
pub mod v1alpha1;
