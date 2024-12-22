/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `kafka.strimzi.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
