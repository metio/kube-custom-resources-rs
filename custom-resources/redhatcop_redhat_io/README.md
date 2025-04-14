<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# redhatcop.redhat.io

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `redhatcop.redhat.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### redhatcop.redhat.io/v1alpha1
- `KeepalivedGroup`
- `AuthEngineMount`
- `AzureAuthEngineConfig`
- `AzureAuthEngineRole`
- `AzureSecretEngineConfig`
- `AzureSecretEngineRole`
- `CertAuthEngineConfig`
- `CertAuthEngineRole`
- `DatabaseSecretEngineConfig`
- `DatabaseSecretEngineRole`
- `DatabaseSecretEngineStaticRole`
- `GCPAuthEngineConfig`
- `GCPAuthEngineRole`
- `GitHubSecretEngineConfig`
- `GitHubSecretEngineRole`
- `GroupAlias`
- `Group`
- `JWTOIDCAuthEngineConfig`
- `JWTOIDCAuthEngineRole`
- `KubernetesAuthEngineConfig`
- `KubernetesAuthEngineRole`
- `KubernetesSecretEngineConfig`
- `KubernetesSecretEngineRole`
- `LDAPAuthEngineConfig`
- `LDAPAuthEngineGroup`
- `PasswordPolicy`
- `PKISecretEngineConfig`
- `PKISecretEngineRole`
- `Policy`
- `QuaySecretEngineConfig`
- `QuaySecretEngineRole`
- `QuaySecretEngineStaticRole`
- `RabbitMQSecretEngineConfig`
- `RabbitMQSecretEngineRole`
- `RandomSecret`
- `SecretEngineMount`
- `VaultSecret`
