/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# cloudwatch_aws_amazon_com

Custom resources in this crate belong to the `cloudwatch.aws.amazon.com` group. The following versions and custom resources are available:

## cloudwatch.aws.amazon.com/v1alpha1
- `AmazonCloudWatchAgent`
- `Instrumentation`
*/
pub mod v1alpha1;
