/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `zonecontrol.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## zonecontrol.k8s.aws/v1
- `ZoneAwareUpdate`
- `ZoneDisruptionBudget`
*/
pub mod v1;
