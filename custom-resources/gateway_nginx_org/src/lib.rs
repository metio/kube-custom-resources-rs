/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `gateway.nginx.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## gateway.nginx.org/v1alpha1
- `ClientSettingsPolicy`
- `NginxGateway`
- `NginxProxy`
- `ObservabilityPolicy`
*/
pub mod v1alpha1;
