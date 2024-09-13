// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/percona/everest-operator/everest.percona.com/v1alpha1/backupstorages.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackupStorageSpec defines the desired state of BackupStorage.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "everest.percona.com", version = "v1alpha1", kind = "BackupStorage", plural = "backupstorages")]
#[kube(namespaced)]
#[kube(status = "BackupStorageStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct BackupStorageSpec {
    /// AllowedNamespaces is the list of namespaces where the operator will copy secrets provided in the CredentialsSecretsName.
    /// 
    /// 
    /// Deprecated: BackupStorages are now used only in the namespaces where they are created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedNamespaces")]
    pub allowed_namespaces: Option<Vec<String>>,
    /// Bucket is a name of bucket.
    pub bucket: String,
    /// CredentialsSecretName is the name of the secret with credentials.
    #[serde(rename = "credentialsSecretName")]
    pub credentials_secret_name: String,
    /// Description stores description of a backup storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// EndpointURL is an endpoint URL of backup storage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointURL")]
    pub endpoint_url: Option<String>,
    /// ForcePathStyle is set to use path-style URLs.
    /// If unspecified, the default value is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forcePathStyle")]
    pub force_path_style: Option<bool>,
    /// Region is a region where the bucket is located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Type is a type of backup storage.
    #[serde(rename = "type")]
    pub r#type: BackupStorageType,
    /// VerifyTLS is set to ensure TLS/SSL verification.
    /// If unspecified, the default value is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyTLS")]
    pub verify_tls: Option<bool>,
}

/// BackupStorageSpec defines the desired state of BackupStorage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupStorageType {
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "azure")]
    Azure,
}

/// BackupStorageStatus defines the observed state of BackupStorage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStorageStatus {
    /// Deprecated: BackupStorages are now used only in the namespaces where they are created.
    #[serde(rename = "usedNamespaces")]
    pub used_namespaces: BTreeMap<String, bool>,
}

