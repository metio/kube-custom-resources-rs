/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# spv_no

Custom resources in this crate belong to the `spv.no` group. The following versions and custom resources are available:

## spv.no/v1
- `AzureKeyVaultSecret`
## spv.no/v1alpha1
- `AzureKeyVaultIdentity`
- `AzureKeyVaultSecret`
- `AzureManagedIdentity`
## spv.no/v2alpha1
- `AzureKeyVaultSecret`
## spv.no/v2beta1
- `AzureKeyVaultSecret`
*/
pub mod v1;
pub mod v1alpha1;
pub mod v2alpha1;
pub mod v2beta1;
