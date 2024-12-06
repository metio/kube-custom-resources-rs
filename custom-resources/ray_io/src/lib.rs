/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `ray.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## ray.io/v1
- `RayCluster`
- `RayJob`
- `RayService`
## ray.io/v1alpha1
- `RayCluster`
- `RayJob`
- `RayService`
*/
pub mod v1;
pub mod v1alpha1;
