<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# pkg.crossplane.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `pkg.crossplane.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### pkg.crossplane.io/v1
- `ConfigurationRevision`
- `Configuration`
- `ProviderRevision`
- `Provider`
### pkg.crossplane.io/v1alpha1
- `ControllerConfig`
### pkg.crossplane.io/v1beta1
- `Lock`
