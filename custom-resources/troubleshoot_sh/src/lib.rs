/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `troubleshoot.sh` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## troubleshoot.sh/v1beta2
- `Analyzer`
- `Collector`
- `HostCollector`
- `HostPreflight`
- `Preflight`
- `Redactor`
- `RemoteCollector`
- `SupportBundle`
*/
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
