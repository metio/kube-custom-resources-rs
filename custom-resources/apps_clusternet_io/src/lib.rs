/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# apps_clusternet_io

Custom resources in this crate belong to the `apps.clusternet.io` group. The following versions and custom resources are available:

## apps.clusternet.io/v1alpha1
- `Base`
- `Description`
- `FeedInventory`
- `Globalization`
- `HelmChart`
- `HelmRelease`
- `Localization`
- `Manifest`
- `Subscription`
*/
pub mod v1alpha1;
