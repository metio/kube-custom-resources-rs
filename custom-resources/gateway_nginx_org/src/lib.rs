/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# gateway_nginx_org

Custom resources in this crate belong to the `gateway.nginx.org` group. The following versions and custom resources are available:

## gateway.nginx.org/v1alpha1
- `ClientSettingsPolicy`
- `NginxGateway`
- `NginxProxy`
- `ObservabilityPolicy`
*/
pub mod v1alpha1;
