/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `image.toolkit.fluxcd.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## image.toolkit.fluxcd.io/v1beta1
- `ImageUpdateAutomation`
- `ImagePolicy`
- `ImageRepository`
## image.toolkit.fluxcd.io/v1beta2
- `ImageUpdateAutomation`
- `ImagePolicy`
- `ImageRepository`
*/
pub mod v1beta1;
pub mod v1beta2;
