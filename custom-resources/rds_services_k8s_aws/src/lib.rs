/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# rds_services_k8s_aws

Custom resources in this crate belong to the `rds.services.k8s.aws` group. The following versions and custom resources are available:

## rds.services.k8s.aws/v1alpha1
- `DBClusterParameterGroup`
- `DBCluster`
- `DBInstance`
- `DBParameterGroup`
- `DBProxy`
- `DBSubnetGroup`
- `GlobalCluster`
*/
pub mod v1alpha1;
