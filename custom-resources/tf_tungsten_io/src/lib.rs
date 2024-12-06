/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `tf.tungsten.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
