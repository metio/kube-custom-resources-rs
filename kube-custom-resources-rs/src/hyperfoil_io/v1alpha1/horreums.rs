// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Hyperfoil/horreum-operator/hyperfoil.io/v1alpha1/horreums.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// HorreumSpec defines the desired state of Horreum
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hyperfoil.io", version = "v1alpha1", kind = "Horreum", plural = "horreums")]
#[kube(namespaced)]
#[kube(status = "HorreumStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HorreumSpec {
    /// Name of secret resource with data `username` and `password`. This will be the first user that get's created in Horreum with the `admin` role, therefore it can create other users and teams. Created automatically if it does not exist.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminSecret")]
    pub admin_secret: Option<String>,
    /// Database coordinates for Horreum data. Besides `username` and `password` the secret must also contain key `dbsecret` that will be used to sign access to the database.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<HorreumDatabase>,
    /// Horreum image. Defaults to quay.io/hyperfoil/horreum:latest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Keycloak specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keycloak: Option<HorreumKeycloak>,
    /// Host used for NodePort services
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeHost")]
    pub node_host: Option<String>,
    /// PostgreSQL specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgres: Option<HorreumPostgres>,
    /// Route for external access
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<HorreumRoute>,
    /// Alternative service type when routes are not available (e.g. on vanilla K8s)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<String>,
}

/// Database coordinates for Horreum data. Besides `username` and `password` the secret must also contain key `dbsecret` that will be used to sign access to the database.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumDatabase {
    /// Hostname for the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Name of the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Database port; defaults to 5432
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Name of secret resource with data `username` and `password`. Created if does not exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// Keycloak specification
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumKeycloak {
    /// Secret used for admin access to the deployed Keycloak instance. Created if does not exist. Must contain keys `username` and `password`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminSecret")]
    pub admin_secret: Option<String>,
    /// Database coordinates Keycloak should use
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<HorreumKeycloakDatabase>,
    /// When this is set Keycloak instance will not be deployed and Horreum will use this external instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external: Option<HorreumKeycloakExternal>,
    /// Image that should be used for Keycloak deployment. Defaults to quay.io/keycloak/keycloak:latest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Route for external access to the Keycloak instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<HorreumKeycloakRoute>,
    /// Alternative service type when routes are not available (e.g. on vanilla K8s)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<String>,
}

/// Database coordinates Keycloak should use
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumKeycloakDatabase {
    /// Hostname for the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Name of the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Database port; defaults to 5432
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Name of secret resource with data `username` and `password`. Created if does not exist.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// When this is set Keycloak instance will not be deployed and Horreum will use this external instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumKeycloakExternal {
    /// Internal URI - Horreum will use this for communication but won't disclose that.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "internalUri")]
    pub internal_uri: Option<String>,
    /// Public facing URI - Horreum will send this URI to the clients.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicUri")]
    pub public_uri: Option<String>,
}

/// Route for external access to the Keycloak instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumKeycloakRoute {
    /// Host for the route leading to Controller REST endpoint. Example: horreum.apps.mycloud.example.com
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Optional for edge and reencrypt routes, required for passthrough; Name of the secret hosting `tls.crt`, `tls.key` and optionally `ca.crt`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    /// Either 'http' (for plain-text routes - not recommended), 'edge', 'reencrypt' or 'passthrough'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// PostgreSQL specification
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumPostgres {
    /// Secret used for unrestricted access to the database. Created if does not exist. Must contain keys `username` and `password`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminSecret")]
    pub admin_secret: Option<String>,
    /// True (or omitted) to deploy PostgreSQL database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Image used for PostgreSQL deployment. Defaults to registry.redhat.io/rhel8/postgresql-12:latest
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Name of PVC where the database will store the data. If empty, ephemeral storage will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<String>,
    /// Id of the user the container should run as
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<i64>,
}

/// Route for external access
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumRoute {
    /// Host for the route leading to Controller REST endpoint. Example: horreum.apps.mycloud.example.com
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Optional for edge and reencrypt routes, required for passthrough; Name of the secret hosting `tls.crt`, `tls.key` and optionally `ca.crt`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    /// Either 'http' (for plain-text routes - not recommended), 'edge', 'reencrypt' or 'passthrough'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// HorreumStatus defines the observed state of Horreum
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HorreumStatus {
    /// Public URL of Keycloak
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keycloakUrl")]
    pub keycloak_url: Option<String>,
    /// Last time state has changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdate")]
    pub last_update: Option<String>,
    /// Public URL of the Horreum application
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicUrl")]
    pub public_url: Option<String>,
    /// Explanation for the current status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Ready, Pending or Error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

