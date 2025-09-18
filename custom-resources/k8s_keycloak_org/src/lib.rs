/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `k8s.keycloak.org` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## k8s.keycloak.org/v2alpha1
- `KeycloakRealmImport`
- `Keycloak`
*/
#[cfg(feature = "v2alpha1")]
pub mod v2alpha1;
