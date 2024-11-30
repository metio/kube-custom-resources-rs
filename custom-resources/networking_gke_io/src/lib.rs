/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# networking_gke_io

Custom resources in this crate belong to the `networking.gke.io` group. The following versions and custom resources are available:

## networking.gke.io/v1
- `ManagedCertificate`
- `GCPBackendPolicy`
- `GCPGatewayPolicy`
- `HealthCheckPolicy`
- `LBPolicy`
*/
pub mod v1;
