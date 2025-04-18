// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/datainfrahq/druid-operator/druid.apache.org/v1alpha1/druidingestions.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "druid.apache.org", version = "v1alpha1", kind = "DruidIngestion", plural = "druidingestions")]
#[kube(namespaced)]
#[kube(status = "DruidIngestionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DruidIngestionSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<DruidIngestionAuth>,
    #[serde(rename = "druidCluster")]
    pub druid_cluster: String,
    pub ingestion: DruidIngestionIngestion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidIngestionAuth {
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret
    /// in any namespace
    #[serde(rename = "secretRef")]
    pub secret_ref: DruidIngestionAuthSecretRef,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret
/// in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidIngestionAuthSecretRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidIngestionIngestion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<BTreeMap<String, serde_json::Value>>,
    /// nativeSpec allows the ingestion specification to be defined in a native Kubernetes format.
    /// This is particularly useful for environment-specific configurations and will eventually
    /// replace the JSON-based Spec field.
    /// Note: Spec will be ignored if nativeSpec is provided.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nativeSpec")]
    pub native_spec: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DruidIngestionIngestionRules>>,
    /// Spec should be passed in as a JSON string.
    /// Note: This field is planned for deprecation in favor of nativeSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidIngestionStatus {
    /// CurrentIngestionSpec is a string instead of RawExtension to maintain compatibility with existing
    /// IngestionSpecs that are stored as JSON strings.
    #[serde(rename = "currentIngestionSpec.json")]
    pub current_ingestion_spec_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateTime")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DruidIngestionStatusRules>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

