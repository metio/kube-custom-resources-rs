/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# secrets_hashicorp_com

Custom resources in this crate belong to the `secrets.hashicorp.com` group. The following versions and custom resources are available:

## secrets.hashicorp.com/v1beta1
- `HCPAuth`
- `HCPVaultSecretsApp`
- `VaultAuth`
- `VaultConnection`
- `VaultDynamicSecret`
- `VaultPKISecret`
- `VaultStaticSecret`
*/
pub mod v1beta1;
