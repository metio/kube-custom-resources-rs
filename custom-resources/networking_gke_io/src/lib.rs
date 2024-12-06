/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `networking.gke.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## networking.gke.io/v1
- `ManagedCertificate`
- `GCPBackendPolicy`
- `GCPGatewayPolicy`
- `HealthCheckPolicy`
- `LBPolicy`
*/
pub mod v1;
