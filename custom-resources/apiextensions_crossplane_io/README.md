<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# apiextensions.crossplane.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `apiextensions.crossplane.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### apiextensions.crossplane.io/v1
- `CompositeResourceDefinition`
- `CompositionRevision`
- `Composition`
### apiextensions.crossplane.io/v1beta1
- `CompositionRevision`
### apiextensions.crossplane.io/v2
- `CompositeResourceDefinition`
### apiextensions.crossplane.io/v2alpha1
- `CompositeResourceDefinition`
