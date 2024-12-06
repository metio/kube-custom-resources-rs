/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `k8s.nginx.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## k8s.nginx.org/v1
- `GlobalConfiguration`
- `Policy`
- `TransportServer`
- `VirtualServerRoute`
- `VirtualServer`
## k8s.nginx.org/v1alpha1
- `GlobalConfiguration`
- `Policy`
- `TransportServer`
*/
pub mod v1;
pub mod v1alpha1;
