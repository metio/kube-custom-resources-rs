/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `camel.apache.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## camel.apache.org/v1
- `Build`
- `CamelCatalog`
- `Kamelet`
## camel.apache.org/v1alpha1
- `Kamelet`
*/
pub mod v1;
pub mod v1alpha1;
