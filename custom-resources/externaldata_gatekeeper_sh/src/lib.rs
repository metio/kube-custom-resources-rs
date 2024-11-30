/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# externaldata_gatekeeper_sh

Custom resources in this crate belong to the `externaldata.gatekeeper.sh` group. The following versions and custom resources are available:

## externaldata.gatekeeper.sh/v1alpha1
- `Provider`
## externaldata.gatekeeper.sh/v1beta1
- `Provider`
*/
pub mod v1alpha1;
pub mod v1beta1;
