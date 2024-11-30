/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kafka_banzaicloud_io

Custom resources in this crate belong to the `kafka.banzaicloud.io` group. The following versions and custom resources are available:

## kafka.banzaicloud.io/v1alpha1
- `CruiseControlOperation`
- `KafkaTopic`
- `KafkaUser`
## kafka.banzaicloud.io/v1beta1
- `KafkaCluster`
*/
pub mod v1alpha1;
pub mod v1beta1;
