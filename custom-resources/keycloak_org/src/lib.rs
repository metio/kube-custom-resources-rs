/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# keycloak_org

Custom resources in this crate belong to the `keycloak.org` group. The following versions and custom resources are available:

## keycloak.org/v1alpha1
- `KeycloakBackup`
- `KeycloakClient`
- `KeycloakRealm`
- `Keycloak`
- `KeycloakUser`
*/
pub mod v1alpha1;
