// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/percona/everest-operator/everest.percona.com/v1alpha1/databaseclusters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DatabaseClusterSpec defines the desired state of DatabaseCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "everest.percona.com", version = "v1alpha1", kind = "DatabaseCluster", plural = "databaseclusters")]
#[kube(namespaced)]
#[kube(status = "DatabaseClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct DatabaseClusterSpec {
    /// AllowUnsafeConfiguration field used to ensure that the user can create configurations unfit for production use.
    /// 
    /// Deprecated: AllowUnsafeConfiguration will not be supported in the future releases.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowUnsafeConfiguration")]
    pub allow_unsafe_configuration: Option<bool>,
    /// Backup is the backup specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<DatabaseClusterBackup>,
    /// DataSource defines a data source for bootstraping a new cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSource")]
    pub data_source: Option<DatabaseClusterDataSource>,
    /// Engine is the database engine specification
    pub engine: DatabaseClusterEngine,
    /// Monitoring is the monitoring configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<DatabaseClusterMonitoring>,
    /// Paused is a flag to stop the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Proxy is the proxy specification. If not set, an appropriate
    /// proxy specification will be applied for the given engine. A
    /// common use case for setting this field is to control the
    /// external access to the database cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<DatabaseClusterProxy>,
    /// Sharding is the sharding configuration. PSMDB-only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sharding: Option<DatabaseClusterSharding>,
}

/// Backup is the backup specification
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterBackup {
    /// Enabled is a flag to enable backups
    /// Deprecated. Please use db.spec.backup.schedules[].enabled to control each schedule separately and db.spec.backup.pitr.enabled to control PITR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// PITR is the configuration of the point in time recovery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitr: Option<DatabaseClusterBackupPitr>,
    /// Schedules is a list of backup schedules
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<DatabaseClusterBackupSchedules>>,
}

/// PITR is the configuration of the point in time recovery
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterBackupPitr {
    /// BackupStorageName is the name of the BackupStorage where the PITR is enabled
    /// The BackupStorage must be created in the same namespace as the DatabaseCluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupStorageName")]
    pub backup_storage_name: Option<String>,
    /// Enabled is a flag to enable PITR
    pub enabled: bool,
    /// UploadIntervalSec number of seconds between the binlogs uploads
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploadIntervalSec")]
    pub upload_interval_sec: Option<i64>,
}

/// BackupSchedule is the backup schedule configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterBackupSchedules {
    /// BackupStorageName is the name of the BackupStorage CR that defines the
    /// storage location.
    /// The BackupStorage must be created in the same namespace as the DatabaseCluster.
    #[serde(rename = "backupStorageName")]
    pub backup_storage_name: String,
    /// Enabled is a flag to enable the schedule
    pub enabled: bool,
    /// Name is the name of the schedule
    pub name: String,
    /// RetentionCopies is the number of backup copies to retain
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionCopies")]
    pub retention_copies: Option<i32>,
    /// Schedule is the cron schedule
    pub schedule: String,
}

/// DataSource defines a data source for bootstraping a new cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterDataSource {
    /// BackupSource is the backup source to restore from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSource")]
    pub backup_source: Option<DatabaseClusterDataSourceBackupSource>,
    /// DBClusterBackupName is the name of the DB cluster backup to restore from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbClusterBackupName")]
    pub db_cluster_backup_name: Option<String>,
    /// PITR is the point-in-time recovery configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pitr: Option<DatabaseClusterDataSourcePitr>,
}

/// BackupSource is the backup source to restore from
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterDataSourceBackupSource {
    /// BackupStorageName is the name of the BackupStorage used for backups.
    /// The BackupStorage must be created in the same namespace as the DatabaseCluster.
    #[serde(rename = "backupStorageName")]
    pub backup_storage_name: String,
    /// Path is the path to the backup file/directory.
    pub path: String,
}

/// PITR is the point-in-time recovery configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterDataSourcePitr {
    /// Date is the UTC date to recover to. The accepted format: "2006-01-02T15:04:05Z".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Type is the type of recovery.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DatabaseClusterDataSourcePitrType>,
}

/// PITR is the point-in-time recovery configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatabaseClusterDataSourcePitrType {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "latest")]
    Latest,
}

/// Engine is the database engine specification
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DatabaseClusterEngine {
    /// Config is the engine configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// CRVersion is the desired version of the CR to use with the
    /// underlying operator.
    /// If unspecified, everest-operator will use the same version as the operator.
    /// 
    /// NOTE: Updating this property post installation may lead to a restart of the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crVersion")]
    pub cr_version: Option<String>,
    /// Replicas is the number of engine replicas
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resources are the resource limits for each engine replica.
    /// If not set, resource limits are not imposed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<DatabaseClusterEngineResources>,
    /// Storage is the engine storage configuration
    pub storage: DatabaseClusterEngineStorage,
    /// Type is the engine type
    #[serde(rename = "type")]
    pub r#type: DatabaseClusterEngineType,
    /// UserSecretsName is the name of the secret containing the user secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userSecretsName")]
    pub user_secrets_name: Option<String>,
    /// Version is the engine version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Resources are the resource limits for each engine replica.
/// If not set, resource limits are not imposed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterEngineResources {
    /// CPU is the CPU resource requirements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<IntOrString>,
    /// Memory is the memory resource requirements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<IntOrString>,
}

/// Storage is the engine storage configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterEngineStorage {
    /// Class is the storage class to use for the persistent volume claim
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// Size is the size of the persistent volume claim
    pub size: IntOrString,
}

/// Engine is the database engine specification
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatabaseClusterEngineType {
    #[serde(rename = "pxc")]
    Pxc,
    #[serde(rename = "postgresql")]
    Postgresql,
    #[serde(rename = "psmdb")]
    Psmdb,
}

/// Monitoring is the monitoring configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterMonitoring {
    /// MonitoringConfigName is the name of a monitoringConfig CR.
    /// The MonitoringConfig must be created in the same namespace as the DatabaseCluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringConfigName")]
    pub monitoring_config_name: Option<String>,
    /// Resources defines resource limitations for the monitoring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<DatabaseClusterMonitoringResources>,
}

/// Resources defines resource limitations for the monitoring.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterMonitoringResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<DatabaseClusterMonitoringResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterMonitoringResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available, otherwise
    /// only the result of this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

/// Proxy is the proxy specification. If not set, an appropriate
/// proxy specification will be applied for the given engine. A
/// common use case for setting this field is to control the
/// external access to the database cluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterProxy {
    /// Config is the proxy configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Expose is the proxy expose configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expose: Option<DatabaseClusterProxyExpose>,
    /// Replicas is the number of proxy replicas
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resources are the resource limits for each proxy replica.
    /// If not set, resource limits are not imposed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<DatabaseClusterProxyResources>,
    /// Type is the proxy type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DatabaseClusterProxyType>,
}

/// Expose is the proxy expose configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterProxyExpose {
    /// IPSourceRanges is the list of IP source ranges (CIDR notation)
    /// to allow access from. If not set, there is no limitations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipSourceRanges")]
    pub ip_source_ranges: Option<Vec<String>>,
    /// Type is the expose type, can be internal or external
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<DatabaseClusterProxyExposeType>,
}

/// Expose is the proxy expose configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatabaseClusterProxyExposeType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
}

/// Resources are the resource limits for each proxy replica.
/// If not set, resource limits are not imposed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterProxyResources {
    /// CPU is the CPU resource requirements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<IntOrString>,
    /// Memory is the memory resource requirements
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<IntOrString>,
}

/// Proxy is the proxy specification. If not set, an appropriate
/// proxy specification will be applied for the given engine. A
/// common use case for setting this field is to control the
/// external access to the database cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DatabaseClusterProxyType {
    #[serde(rename = "mongos")]
    Mongos,
    #[serde(rename = "haproxy")]
    Haproxy,
    #[serde(rename = "proxysql")]
    Proxysql,
    #[serde(rename = "pgbouncer")]
    Pgbouncer,
}

/// Sharding is the sharding configuration. PSMDB-only
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterSharding {
    /// ConfigServer represents the sharding configuration server settings
    #[serde(rename = "configServer")]
    pub config_server: DatabaseClusterShardingConfigServer,
    /// Enabled defines if the sharding is enabled
    pub enabled: bool,
    /// Shards defines the number of shards
    pub shards: i32,
}

/// ConfigServer represents the sharding configuration server settings
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterShardingConfigServer {
    /// Replicas is the amount of configServers
    pub replicas: i32,
}

/// DatabaseClusterStatus defines the observed state of DatabaseCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseClusterStatus {
    /// ActiveStorage is the storage used in cluster (psmdb only)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeStorage")]
    pub active_storage: Option<String>,
    /// Conditions contains the observed conditions of the DatabaseCluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// CRVersion is the observed version of the CR used with the underlying operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crVersion")]
    pub cr_version: Option<String>,
    /// Details provides full status of the upstream cluster as a plain text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Hostname is the hostname where the cluster can be reached
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Message is extra information about the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// ObservedGeneration is the most recent generation observed for this DatabaseCluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Port is the port where the cluster can be reached
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Ready is the number of ready pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    /// RecommendedCRVersion is the recommended version of the CR to use.
    /// If set, the CR needs to be updated to this version before upgrading the operator.
    /// If unset, the CR is already at the recommended version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recommendedCRVersion")]
    pub recommended_cr_version: Option<String>,
    /// Size is the total number of pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Status is the status of the cluster
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

