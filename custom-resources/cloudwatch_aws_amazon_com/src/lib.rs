/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `cloudwatch.aws.amazon.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## cloudwatch.aws.amazon.com/v1alpha1
- `AmazonCloudWatchAgent`
- `Instrumentation`
*/
pub mod v1alpha1;
