<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# kafka.strimzi.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `kafka.strimzi.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### kafka.strimzi.io/v1alpha1
- `KafkaTopic`
- `KafkaUser`
### kafka.strimzi.io/v1beta1
- `KafkaTopic`
- `KafkaUser`
### kafka.strimzi.io/v1beta2
- `KafkaBridge`
- `KafkaConnector`
- `KafkaConnect`
- `KafkaMirrorMaker2`
- `KafkaMirrorMaker`
- `KafkaNodePool`
- `KafkaRebalance`
- `Kafka`
- `KafkaTopic`
- `KafkaUser`
