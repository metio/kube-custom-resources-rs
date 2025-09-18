/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `resources.teleport.dev` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

## resources.teleport.dev/v1
- `TeleportLoginRule`
- `TeleportOktaImportRule`
## resources.teleport.dev/v2
- `TeleportProvisionToken`
- `TeleportSAMLConnector`
- `TeleportUser`
## resources.teleport.dev/v3
- `TeleportGithubConnector`
- `TeleportOIDCConnector`
## resources.teleport.dev/v5
- `TeleportRole`
## resources.teleport.dev/v6
- `TeleportRole`
*/
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;
#[cfg(feature = "v3")]
pub mod v3;
#[cfg(feature = "v5")]
pub mod v5;
#[cfg(feature = "v6")]
pub mod v6;
