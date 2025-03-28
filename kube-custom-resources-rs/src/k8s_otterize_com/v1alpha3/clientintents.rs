// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/otterize/helm-charts/k8s.otterize.com/v1alpha3/clientintents.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// IntentsSpec defines the desired state of ClientIntents
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8s.otterize.com", version = "v1alpha3", kind = "ClientIntents", plural = "clientintents")]
#[kube(namespaced)]
#[kube(status = "ClientIntentsStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClientIntentsSpec {
    pub calls: Vec<ClientIntentsCalls>,
    pub service: ClientIntentsService,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCalls {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "HTTPResources")]
    pub http_resources: Option<Vec<ClientIntentsCallsHttpResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsActions")]
    pub aws_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "azureKeyVaultPolicy")]
    pub azure_key_vault_policy: Option<ClientIntentsCallsAzureKeyVaultPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "azureRoles")]
    pub azure_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseResources")]
    pub database_resources: Option<Vec<ClientIntentsCallsDatabaseResources>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcpPermissions")]
    pub gcp_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internet: Option<ClientIntentsCallsInternet>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kafkaTopics")]
    pub kafka_topics: Option<Vec<ClientIntentsCallsKafkaTopics>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ClientIntentsCallsType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCallsHttpResources {
    pub methods: Vec<String>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCallsAzureKeyVaultPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificatePermissions")]
    pub certificate_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyPermissions")]
    pub key_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretPermissions")]
    pub secret_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storagePermissions")]
    pub storage_permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCallsDatabaseResources {
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCallsInternet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsCallsKafkaTopics {
    pub name: String,
    pub operations: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientIntentsCallsType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "kafka")]
    Kafka,
    #[serde(rename = "database")]
    Database,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "internet")]
    Internet,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsService {
    pub name: String,
}

/// IntentsStatus defines the observed state of ClientIntents
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsStatus {
    /// The last generation of the intents that was successfully reconciled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedIPs")]
    pub resolved_i_ps: Option<Vec<ClientIntentsStatusResolvedIPs>>,
    /// upToDate field reflects whether the client intents have successfully been applied
    /// to the cluster to the state specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upToDate")]
    pub up_to_date: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClientIntentsStatusResolvedIPs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<String>>,
}

