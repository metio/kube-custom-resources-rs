/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `api.clever-cloud.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
