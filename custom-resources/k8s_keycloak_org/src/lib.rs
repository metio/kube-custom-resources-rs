/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# k8s_keycloak_org

Custom resources in this crate belong to the `k8s.keycloak.org` group. The following versions and custom resources are available:

## k8s.keycloak.org/v2alpha1
- `KeycloakRealmImport`
- `Keycloak`
*/
pub mod v2alpha1;
