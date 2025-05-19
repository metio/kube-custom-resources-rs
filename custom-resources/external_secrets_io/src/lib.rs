/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `external-secrets.io` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## external-secrets.io/v1
- `ClusterExternalSecret`
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`
## external-secrets.io/v1alpha1
- `ClusterPushSecret`
- `ClusterSecretStore`
- `ExternalSecret`
- `PushSecret`
- `SecretStore`
## external-secrets.io/v1beta1
- `ClusterExternalSecret`
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
