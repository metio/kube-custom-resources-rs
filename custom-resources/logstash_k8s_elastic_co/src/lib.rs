/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `logstash.k8s.elastic.co` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## logstash.k8s.elastic.co/v1alpha1
- `Logstash`
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
