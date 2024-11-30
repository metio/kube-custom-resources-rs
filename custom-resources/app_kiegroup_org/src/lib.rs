/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# app_kiegroup_org

Custom resources in this crate belong to the `app.kiegroup.org` group. The following versions and custom resources are available:

## app.kiegroup.org/v1beta1
- `KogitoBuild`
- `KogitoInfra`
- `KogitoRuntime`
- `KogitoSupportingService`
*/
pub mod v1beta1;
