/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `redhatcop.redhat.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## redhatcop.redhat.io/v1alpha1
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
*/
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
