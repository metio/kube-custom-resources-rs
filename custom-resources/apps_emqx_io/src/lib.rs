/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# apps_emqx_io

Custom resources in this crate belong to the `apps.emqx.io` group. The following versions and custom resources are available:

## apps.emqx.io/v1beta3
- `EmqxBroker`
- `EmqxEnterprise`
- `EmqxPlugin`
## apps.emqx.io/v1beta4
- `EmqxBroker`
- `EmqxEnterprise`
- `EmqxPlugin`
- `Rebalance`
## apps.emqx.io/v2alpha1
- `EMQX`
## apps.emqx.io/v2beta1
- `EMQX`
- `Rebalance`
*/
pub mod v1beta3;
pub mod v1beta4;
pub mod v2alpha1;
pub mod v2beta1;
