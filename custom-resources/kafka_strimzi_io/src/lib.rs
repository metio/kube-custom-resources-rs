/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# kafka_strimzi_io

Custom resources in this crate belong to the `kafka.strimzi.io` group. The following versions and custom resources are available:

## kafka.strimzi.io/v1alpha1
- `KafkaTopic`
- `KafkaUser`
## kafka.strimzi.io/v1beta1
- `KafkaTopic`
- `KafkaUser`
## kafka.strimzi.io/v1beta2
- `KafkaBridge`
- `KafkaConnector`
- `KafkaConnect`
- `KafkaMirrorMaker`
- `KafkaNodePool`
- `KafkaRebalance`
- `Kafka`
- `KafkaTopic`
- `KafkaUser`
*/
pub mod v1alpha1;
pub mod v1beta1;
pub mod v1beta2;
