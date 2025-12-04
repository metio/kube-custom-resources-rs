/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `operator.victoriametrics.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## operator.victoriametrics.com/v1
- `VLAgent`
- `VLCluster`
- `VLSingle`
- `VMAnomaly`
- `VTCluster`
- `VTSingle`
## operator.victoriametrics.com/v1beta1
- `VLogs`
- `VMAgent`
- `VMAlertmanagerConfig`
- `VMAlertmanager`
- `VMAlert`
- `VMAuth`
- `VMCluster`
- `VMNodeScrape`
- `VMPodScrape`
- `VMProbe`
- `VMRule`
- `VMScrapeConfig`
- `VMServiceScrape`
- `VMSingle`
- `VMStaticScrape`
- `VMUser`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
