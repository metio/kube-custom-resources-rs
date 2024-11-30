/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# prometheusservice_services_k8s_aws

Custom resources in this crate belong to the `prometheusservice.services.k8s.aws` group. The following versions and custom resources are available:

## prometheusservice.services.k8s.aws/v1alpha1
- `AlertManagerDefinition`
- `LoggingConfiguration`
- `RuleGroupsNamespace`
- `Workspace`
*/
pub mod v1alpha1;
