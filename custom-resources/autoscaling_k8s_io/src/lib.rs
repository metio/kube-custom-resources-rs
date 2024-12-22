/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `autoscaling.k8s.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## autoscaling.k8s.io/v1
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`
## autoscaling.k8s.io/v1beta2
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
