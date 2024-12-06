// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/infinispan/infinispan-operator/infinispan.org/v2alpha1/backups.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// BackupSpec defines the desired state of Backup
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infinispan.org", version = "v2alpha1", kind = "Backup", plural = "backups")]
#[kube(namespaced)]
#[kube(status = "BackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupSpec {
    /// Infinispan cluster name
    pub cluster: String,
    /// InfinispanContainerSpec specify resource requirements per container
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<BackupContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<BackupResources>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<BackupVolume>,
}

/// InfinispanContainerSpec specify resource requirements per container
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupContainer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cliExtraJvmOpts")]
    pub cli_extra_jvm_opts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraJvmOpts")]
    pub extra_jvm_opts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routerExtraJvmOpts")]
    pub router_extra_jvm_opts: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupResources {
    /// Deprecated and to be removed on subsequent release. Use .Templates instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cacheConfigs")]
    pub cache_configs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caches: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protoSchemas")]
    pub proto_schemas: Option<Vec<String>>,
    /// Deprecated and to be removed on subsequent release. Use .Tasks instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupVolume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Names the storage class object for persistent volume claims.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
}

/// BackupStatus defines the observed state of Backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStatus {
    /// Current phase of the backup operation
    pub phase: String,
    /// The name of the created PersistentVolumeClaim used to store the backup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pvc: Option<String>,
    /// Reason indicates the reason for any backup related failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

