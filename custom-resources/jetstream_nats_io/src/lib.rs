/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `jetstream.nats.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## jetstream.nats.io/v1beta1
- `Consumer`
- `Stream`
- `StreamTemplate`
## jetstream.nats.io/v1beta2
- `Account`
- `Consumer`
- `KeyValue`
- `ObjectStore`
- `Stream`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
#[cfg(feature = "v1beta2")]
pub mod v1beta2;
