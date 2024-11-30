/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# autoscaling_k8s_io

Custom resources in this crate belong to the `autoscaling.k8s.io` group. The following versions and custom resources are available:

## autoscaling.k8s.io/v1
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`
## autoscaling.k8s.io/v1beta2
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`
*/
pub mod v1;
pub mod v1beta2;
