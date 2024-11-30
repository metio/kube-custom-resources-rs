/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# sagemaker_services_k8s_aws

Custom resources in this crate belong to the `sagemaker.services.k8s.aws` group. The following versions and custom resources are available:

## sagemaker.services.k8s.aws/v1alpha1
- `App`
- `DataQualityJobDefinition`
- `Domain`
- `EndpointConfig`
- `Endpoint`
- `FeatureGroup`
- `HyperParameterTuningJob`
- `ModelBiasJobDefinition`
- `ModelExplainabilityJobDefinition`
- `ModelPackageGroup`
- `ModelPackage`
- `ModelQualityJobDefinition`
- `Model`
- `MonitoringSchedule`
- `NotebookInstanceLifecycleConfig`
- `NotebookInstance`
- `ProcessingJob`
- `TrainingJob`
- `TransformJob`
- `UserProfile`
*/
pub mod v1alpha1;
