/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# api_clever_cloud_com

Custom resources in this crate belong to the `api.clever-cloud.com` group. The following versions and custom resources are available:

## api.clever-cloud.com/v1
- `ConfigProvider`
- `ElasticSearch`
- `MongoDb`
- `MySql`
- `PostgreSql`
- `Redis`
## api.clever-cloud.com/v1beta1
- `Pulsar`
*/
pub mod v1;
pub mod v1beta1;
