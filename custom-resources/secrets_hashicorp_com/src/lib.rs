/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `secrets.hashicorp.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## secrets.hashicorp.com/v1beta1
- `HCPAuth`
- `HCPVaultSecretsApp`
- `VaultAuth`
- `VaultConnection`
- `VaultDynamicSecret`
- `VaultPKISecret`
- `VaultStaticSecret`
*/
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
