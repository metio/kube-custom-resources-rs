/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# tf_tungsten_io

Custom resources in this crate belong to the `tf.tungsten.io` group. The following versions and custom resources are available:

## tf.tungsten.io/v1alpha1
- `Analytics`
- `AnalyticsAlarm`
- `AnalyticsSnmp`
- `Cassandra`
- `Config`
- `Control`
- `Kubemanager`
- `Manager`
- `QueryEngine`
- `Rabbitmq`
- `Redis`
- `Vrouter`
- `Zookeeper`
*/
pub mod v1alpha1;
