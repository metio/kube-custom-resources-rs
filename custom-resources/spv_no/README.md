<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# spv.no

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `spv.no` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### spv.no/v1
- `AzureKeyVaultSecret`
### spv.no/v1alpha1
- `AzureKeyVaultIdentity`
- `AzureKeyVaultSecret`
- `AzureManagedIdentity`
### spv.no/v2alpha1
- `AzureKeyVaultSecret`
### spv.no/v2beta1
- `AzureKeyVaultSecret`
