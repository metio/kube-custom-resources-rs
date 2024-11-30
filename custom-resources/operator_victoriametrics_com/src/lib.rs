/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# operator_victoriametrics_com

Custom resources in this crate belong to the `operator.victoriametrics.com` group. The following versions and custom resources are available:

## operator.victoriametrics.com/v1beta1
- `VMAlertmanagerConfig`
- `VMNodeScrape`
- `VMPodScrape`
- `VMProbe`
- `VMRule`
- `VMScrapeConfig`
- `VMServiceScrape`
- `VMStaticScrape`
- `VMUser`
*/
pub mod v1beta1;
