<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# sagemaker.services.k8s.aws

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `sagemaker.services.k8s.aws` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### sagemaker.services.k8s.aws/v1alpha1
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
