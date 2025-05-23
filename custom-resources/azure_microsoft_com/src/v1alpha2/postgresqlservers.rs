// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha2/postgresqlservers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// PostgreSQLServerSpec defines the desired state of PostgreSQLServer
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha2", kind = "PostgreSQLServer", plural = "postgresqlservers")]
#[kube(namespaced)]
#[kube(status = "PostgreSQLServerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PostgreSQLServerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createMode")]
    pub create_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyVaultToStoreSecrets")]
    pub key_vault_to_store_secrets: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaProperties")]
    pub replica_properties: Option<PostgreSQLServerReplicaProperties>,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// ServerVersion enumerates the values for server version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverVersion")]
    pub server_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PostgreSQLServerSku>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslEnforcement")]
    pub ssl_enforcement: Option<PostgreSQLServerSslEnforcement>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageProfile")]
    pub storage_profile: Option<PostgreSQLServerStorageProfile>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PostgreSQLServerReplicaProperties {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceServerId")]
    pub source_server_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PostgreSQLServerSku {
    /// Capacity - The scale up/out capacity, representing server's compute units.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    /// Family - The family of hardware.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Name - The name of the sku, typically, tier + family + cores, e.g. B_Gen4_1, GP_Gen5_8.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Size - The size code, to be interpreted by resource as appropriate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Tier - The tier of the particular SKU, e.g. Basic. Possible values include: 'Basic', 'GeneralPurpose', 'MemoryOptimized'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<PostgreSQLServerSkuTier>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostgreSQLServerSkuTier {
    Basic,
    GeneralPurpose,
    MemoryOptimized,
}

/// PostgreSQLServerSpec defines the desired state of PostgreSQLServer
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostgreSQLServerSslEnforcement {
    Enabled,
    Disabled,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PostgreSQLServerStorageProfile {
    /// BackupRetentionDays - Backup retention days for the server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupRetentionDays")]
    pub backup_retention_days: Option<i32>,
    /// GeoRedundantBackup - Enable Geo-redundant or not for server backup. Possible values include: 'Enabled', 'Disabled'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "geoRedundantBackup")]
    pub geo_redundant_backup: Option<String>,
    /// StorageAutogrow - Enable Storage Auto Grow. Possible values include: 'Enabled', 'Disabled'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageAutogrow")]
    pub storage_autogrow: Option<PostgreSQLServerStorageProfileStorageAutogrow>,
    /// StorageMB - Max storage allowed for a server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageMB")]
    pub storage_mb: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostgreSQLServerStorageProfileStorageAutogrow {
    Enabled,
    Disabled,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PostgreSQLServerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containsUpdate")]
    pub contains_update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedProvisioning")]
    pub failed_provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flattenedSecrets")]
    pub flattened_secrets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrl")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrlKind")]
    pub polling_url_kind: Option<PostgreSQLServerStatusPollingUrlKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PostgreSQLServerStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

