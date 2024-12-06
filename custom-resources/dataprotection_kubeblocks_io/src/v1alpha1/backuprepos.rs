// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/apecloud/kubeblocks/dataprotection.kubeblocks.io/v1alpha1/backuprepos.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BackupRepoSpec defines the desired state of `BackupRepo`.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "dataprotection.kubeblocks.io", version = "v1alpha1", kind = "BackupRepo", plural = "backuprepos")]
#[kube(status = "BackupRepoStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct BackupRepoSpec {
    /// Specifies the access method of the backup repository.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMethod")]
    pub access_method: Option<BackupRepoAccessMethod>,
    /// Stores the non-secret configuration parameters for the `StorageProvider`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    /// References to the secret that holds the credentials for the `StorageProvider`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<BackupRepoCredential>,
    /// Specifies the prefix of the path for storing backup data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pathPrefix")]
    pub path_prefix: Option<String>,
    /// Specifies reclaim policy of the PV created by this backup repository.
    #[serde(rename = "pvReclaimPolicy")]
    pub pv_reclaim_policy: BackupRepoPvReclaimPolicy,
    /// Specifies the name of the `StorageProvider` used by this backup repository.
    #[serde(rename = "storageProviderRef")]
    pub storage_provider_ref: String,
    /// Specifies the capacity of the PVC created by this backup repository.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeCapacity")]
    pub volume_capacity: Option<IntOrString>,
}

/// BackupRepoSpec defines the desired state of `BackupRepo`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupRepoAccessMethod {
    Mount,
    Tool,
}

/// References to the secret that holds the credentials for the `StorageProvider`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupRepoCredential {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// BackupRepoSpec defines the desired state of `BackupRepo`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupRepoPvReclaimPolicy {
    Delete,
    Retain,
}

/// BackupRepoStatus defines the observed state of `BackupRepo`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupRepoStatus {
    /// Represents the name of the PVC that stores backup data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupPVCName")]
    pub backup_pvc_name: Option<String>,
    /// Provides a detailed description of the current state of the backup repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Refers to the generated secret for the `StorageProvider`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedCSIDriverSecret")]
    pub generated_csi_driver_secret: Option<BackupRepoStatusGeneratedCsiDriverSecret>,
    /// Represents the name of the generated storage class.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedStorageClassName")]
    pub generated_storage_class_name: Option<String>,
    /// Indicates if this backup repository is the default one.\
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isDefault")]
    pub is_default: Option<bool>,
    /// Represents the latest generation of the resource that the controller has observed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Represents the current phase of reconciliation for the backup repository.
    /// Permissible values are PreChecking, Failed, Ready, Deleting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// Represents the name of the secret that contains the configuration for the tool.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "toolConfigSecretName")]
    pub tool_config_secret_name: Option<String>,
}

/// Refers to the generated secret for the `StorageProvider`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupRepoStatusGeneratedCsiDriverSecret {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

