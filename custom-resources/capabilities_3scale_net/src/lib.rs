/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `capabilities.3scale.net` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## capabilities.3scale.net/v1alpha1
- `Tenant`
## capabilities.3scale.net/v1beta1
- `ActiveDoc`
- `Application`
- `Backend`
- `CustomPolicyDefinition`
- `DeveloperAccount`
- `DeveloperUser`
- `OpenAPI`
- `Product`
- `ProxyConfigPromote`
*/
pub mod v1alpha1;
pub mod v1beta1;
