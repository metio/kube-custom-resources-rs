<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# camel.apache.org

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `camel.apache.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### camel.apache.org/v1
- `Build`
- `CamelCatalog`
- `IntegrationKit`
- `IntegrationPlatform`
- `IntegrationProfile`
- `Integration`
- `Kamelet`
- `Pipe`
### camel.apache.org/v1alpha1
- `KameletBinding`
- `Kamelet`
