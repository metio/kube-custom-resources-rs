/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# postgres_operator_crunchydata_com

Custom resources in this crate belong to the `postgres-operator.crunchydata.com` group. The following versions and custom resources are available:

## postgres-operator.crunchydata.com/v1beta1
- `PGAdmin`
- `PGUpgrade`
- `PostgresCluster`
*/
pub mod v1beta1;
