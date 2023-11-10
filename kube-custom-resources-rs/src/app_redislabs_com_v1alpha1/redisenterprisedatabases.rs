// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/RedisLabs/redis-enterprise-k8s-docs/app.redislabs.com/v1alpha1/redisenterprisedatabases.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "app.redislabs.com", version = "v1alpha1", kind = "RedisEnterpriseDatabase", plural = "redisenterprisedatabases")]
#[kube(namespaced)]
#[kube(status = "RedisEnterpriseDatabaseStatus")]
#[kube(schema = "disabled")]
pub struct RedisEnterpriseDatabaseSpec {
    /// Connection/ association to the Active-Active database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeActive")]
    pub active_active: Option<RedisEnterpriseDatabaseActiveActive>,
    /// Settings for database alerts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alertSettings")]
    pub alert_settings: Option<RedisEnterpriseDatabaseAlertSettings>,
    /// Target for automatic database backups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup: Option<RedisEnterpriseDatabaseBackup>,
    /// The Secrets containing TLS Client Certificate to use for Authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientAuthenticationCertificates")]
    pub client_authentication_certificates: Option<Vec<String>>,
    /// Internode encryption (INE) setting. An optional boolean setting, overriding a similar cluster-wide policy. If set to False, INE is guaranteed to be turned off for this DB (regardless of cluster-wide policy). If set to True, INE will be turned on, unless the capability is not supported by the DB ( in such a case we will get an error and database creation will fail). If left unspecified, will be disabled if internode encryption is not supported by the DB (regardless of cluster default). Deleting this property after explicitly setting its value shall have no effect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataInternodeEncryption")]
    pub data_internode_encryption: Option<bool>,
    /// Database port number. TCP port on which the database is available. Will be generated automatically if omitted. can not be changed after creation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databasePort")]
    pub database_port: Option<i64>,
    /// The name of the secret that holds the password to the database (redis databases only). If secret does not exist, it will be created. To define the password, create an opaque secret and set the name in the spec. The password will be taken from the value of the 'password' key. Use an empty string as value within the secret to disable authentication for the database. Notes - For Active-Active databases this secret will not be automatically created, and also, memcached databases must not be set with a value, and a secret/password will not be automatically created for them. Use the memcachedSaslSecretName field to set authentication parameters for memcached databases.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseSecretName")]
    pub database_secret_name: Option<String>,
    /// Is connecting with a default user allowed?
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultUser")]
    pub default_user: Option<bool>,
    /// Database eviction policy. see more https://docs.redislabs.com/latest/rs/administering/database-operations/eviction-policy/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "evictionPolicy")]
    pub eviction_policy: Option<String>,
    /// Whether it is an RoF database or not. Applicable only for databases of type "REDIS". Assumed to be false if left blank.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isRof")]
    pub is_rof: Option<bool>,
    /// Credentials used for binary authentication in memcached databases. The credentials should be saved as an opaque secret and the name of that secret should be configured using this field. For username, use 'username' as the key and the actual username as the value. For password, use 'password' as the key and the actual password as the value. Note that connections are not encrypted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memcachedSaslSecretName")]
    pub memcached_sasl_secret_name: Option<String>,
    /// memory size of database. use formats like 100MB, 0.1GB. minimum value in 100MB. When redis on flash (RoF) is enabled, this value refers to RAM+Flash memory, and it must not be below 1GB.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySize")]
    pub memory_size: Option<String>,
    /// List of modules associated with database. Note - For Active-Active databases this feature is currently in preview. For this feature to take effect for Active-Active databases, set a boolean environment variable with the name "ENABLE_ALPHA_FEATURES" to True. This variable can be set via the redis-enterprise-operator pod spec, or through the operator-environment-config Config Map.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modulesList")]
    pub modules_list: Option<Vec<RedisEnterpriseDatabaseModulesList>>,
    /// OSS Cluster mode option. Note that not all client libraries support OSS cluster mode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ossCluster")]
    pub oss_cluster: Option<bool>,
    /// Database on-disk persistence policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistence: Option<RedisEnterpriseDatabasePersistence>,
    /// The policy used for proxy binding to the endpoint. Supported proxy policies are: single/all-master-shards/all-nodes When left blank, the default value will be chosen according to the value of ossCluster - single if disabled, all-master-shards when enabled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyPolicy")]
    pub proxy_policy: Option<String>,
    /// Whether database should be rack aware. This improves availability - more information: https://docs.redislabs.com/latest/rs/concepts/high-availability/rack-zone-awareness/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rackAware")]
    pub rack_aware: Option<bool>,
    /// Connection to Redis Enterprise Cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redisEnterpriseCluster")]
    pub redis_enterprise_cluster: Option<RedisEnterpriseDatabaseRedisEnterpriseCluster>,
    /// Redis OSS version. For existing databases - Upgrade Redis OSS version. For new databases - the version which the database will be created with. If set to 'major' - will always upgrade to the most recent major Redis version. If set to 'latest' - will always upgrade to the most recent Redis version. Depends on 'redisUpgradePolicy' - if you want to set the value to 'latest' for some databases, you must set redisUpgradePolicy on the cluster before. Possible values are 'major' or 'latest' When using upgrade - make sure to backup the database before. This value is used only for database type 'redis'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redisVersion")]
    pub redis_version: Option<RedisEnterpriseDatabaseRedisVersion>,
    /// What databases to replicate from
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaSources")]
    pub replica_sources: Option<Vec<RedisEnterpriseDatabaseReplicaSources>>,
    /// In-memory database replication. When enabled, database will have replica shard for every master - leading to higher availability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,
    /// The size of the RAM portion of an RoF database. Similarly to "memorySize" use formats like 100MB, 0.1GB. It must be at least 10% of combined memory size (RAM and Flash), as specified by "memorySize".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rofRamSize")]
    pub rof_ram_size: Option<String>,
    /// List of Redis Enteprise ACL and Role bindings to apply
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolesPermissions")]
    pub roles_permissions: Option<Vec<RedisEnterpriseDatabaseRolesPermissions>>,
    /// Number of database server-side shards
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardCount")]
    pub shard_count: Option<i64>,
    /// Control the density of shards - should they reside on as few or as many nodes as possible. Available options are "dense" or "sparse". If left unset, defaults to "dense".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardsPlacement")]
    pub shards_placement: Option<RedisEnterpriseDatabaseShardsPlacement>,
    /// Require SSL authenticated and encrypted connections to the database. enabled - all incoming connections to the Database must use SSL. disabled - no incoming connection to the Database should use SSL. replica_ssl - databases that replicate from this one need to use SSL.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsMode")]
    pub tls_mode: Option<RedisEnterpriseDatabaseTlsMode>,
    /// The type of the database (redis or memcached). Defaults to "redis".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<RedisEnterpriseDatabaseType>,
}

/// Connection/ association to the Active-Active database.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseActiveActive {
    /// The the corresponding Active-Active database name, Redis Enterprise Active Active Database custom resource name, this Resource is associated with. In case this resource is created manually at the active active database creation this field must be filled via the user, otherwise, the operator will assign this field automatically. Note: this feature is currently unsupported.
    pub name: String,
    /// The corresponding participating cluster name, Redis Enterprise Remote Cluster custom resource name, in the Active-Active database, In case this resource is created manually at the active active database creation this field must be filled via the user, otherwise, the operator will assign this field automatically. Note: this feature is currently unsupported.
    #[serde(rename = "participatingClusterName")]
    pub participating_cluster_name: String,
}

/// Settings for database alerts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettings {
    /// Periodic backup has been delayed for longer than specified threshold value [minutes]. -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_backup_delayed: Option<RedisEnterpriseDatabaseAlertSettingsBdbBackupDelayed>,
    /// Active-active source - sync lag is higher than specified threshold value [seconds] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_high_syncer_lag: Option<RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcHighSyncerLag>,
    /// Active-active source - sync has connection error while trying to connect replica source -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_syncer_connection_error: Option<RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcSyncerConnectionError>,
    /// Active-active source - sync encountered in general error -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_syncer_general_error: Option<RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcSyncerGeneralError>,
    /// Latency is higher than specified threshold value [micro-sec] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_high_latency: Option<RedisEnterpriseDatabaseAlertSettingsBdbHighLatency>,
    /// Throughput is higher than specified threshold value [requests / sec.] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_high_throughput: Option<RedisEnterpriseDatabaseAlertSettingsBdbHighThroughput>,
    /// An alert for state-machines that are running for too long -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_long_running_action: Option<RedisEnterpriseDatabaseAlertSettingsBdbLongRunningAction>,
    /// Throughput is lower than specified threshold value [requests / sec.] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_low_throughput: Option<RedisEnterpriseDatabaseAlertSettingsBdbLowThroughput>,
    /// Dataset RAM overhead of a shard has reached the threshold value [% of its RAM limit] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_ram_dataset_overhead: Option<RedisEnterpriseDatabaseAlertSettingsBdbRamDatasetOverhead>,
    /// Percent of values kept in a shard's RAM is lower than [% of its key count] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_ram_values: Option<RedisEnterpriseDatabaseAlertSettingsBdbRamValues>,
    /// Replica-of source - sync lag is higher than specified threshold value [seconds] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_replica_src_high_syncer_lag: Option<RedisEnterpriseDatabaseAlertSettingsBdbReplicaSrcHighSyncerLag>,
    /// Replica-of source - sync has connection error while trying to connect replica source -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_replica_src_syncer_connection_error: Option<RedisEnterpriseDatabaseAlertSettingsBdbReplicaSrcSyncerConnectionError>,
    /// Number of values kept in a shard's RAM is lower than [values] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_shard_num_ram_values: Option<RedisEnterpriseDatabaseAlertSettingsBdbShardNumRamValues>,
    /// Dataset size has reached the threshold value [% of the memory limit] expected fields: -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bdb_size: Option<RedisEnterpriseDatabaseAlertSettingsBdbSize>,
}

/// Periodic backup has been delayed for longer than specified threshold value [minutes]. -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbBackupDelayed {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Active-active source - sync lag is higher than specified threshold value [seconds] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcHighSyncerLag {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Active-active source - sync has connection error while trying to connect replica source -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcSyncerConnectionError {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Active-active source - sync encountered in general error -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbCrdtSrcSyncerGeneralError {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Latency is higher than specified threshold value [micro-sec] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbHighLatency {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Throughput is higher than specified threshold value [requests / sec.] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbHighThroughput {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// An alert for state-machines that are running for too long -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbLongRunningAction {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Throughput is lower than specified threshold value [requests / sec.] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbLowThroughput {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Dataset RAM overhead of a shard has reached the threshold value [% of its RAM limit] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbRamDatasetOverhead {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Percent of values kept in a shard's RAM is lower than [% of its key count] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbRamValues {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Replica-of source - sync lag is higher than specified threshold value [seconds] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbReplicaSrcHighSyncerLag {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Replica-of source - sync has connection error while trying to connect replica source -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbReplicaSrcSyncerConnectionError {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Number of values kept in a shard's RAM is lower than [values] -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbShardNumRamValues {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Dataset size has reached the threshold value [% of the memory limit] expected fields: -Note threshold is commented (allow string/int/float and support backwards compatibility) but is required
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseAlertSettingsBdbSize {
    /// Alert enabled or disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Target for automatic database backups.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abs: Option<RedisEnterpriseDatabaseBackupAbs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ftp: Option<RedisEnterpriseDatabaseBackupFtp>,
    /// GoogleStorage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<RedisEnterpriseDatabaseBackupGcs>,
    /// Backup Interval in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// MountPointStorage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<RedisEnterpriseDatabaseBackupMount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<RedisEnterpriseDatabaseBackupS3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sftp: Option<RedisEnterpriseDatabaseBackupSftp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swift: Option<RedisEnterpriseDatabaseBackupSwift>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupAbs {
    /// The name of the K8s secret that holds ABS credentials. The secret must contain the keys "AccountName" and "AccountKey", and these must hold the corresponding credentials
    #[serde(rename = "absSecretName")]
    pub abs_secret_name: String,
    /// Azure Blob Storage container name.
    pub container: String,
    /// Optional. Azure Blob Storage subdir under container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdir: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupFtp {
    /// a URI of the ftps://[USER[:PASSWORD]@]HOST[:PORT]/PATH[/]
    pub url: String,
}

/// GoogleStorage
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupGcs {
    /// Google Storage bucket name.
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// The name of the K8s secret that holds the Google Cloud Storage credentials. The secret must contain the keys "CLIENT_ID", "PRIVATE_KEY", "PRIVATE_KEY_ID", "CLIENT_EMAIL" and these must hold the corresponding credentials. The keys should correspond to the values in the key JSON.
    #[serde(rename = "gcsSecretName")]
    pub gcs_secret_name: String,
    /// Optional. Google Storage subdir under bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdir: Option<String>,
}

/// MountPointStorage
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupMount {
    /// Path to the local mount point. You must create the mount point on all nodes, and the redislabs:redislabs user must have read and write permissions on the local mount point.
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupS3 {
    /// The name of the K8s secret that holds the AWS credentials. The secret must contain the keys "AWS_ACCESS_KEY_ID" and "AWS_SECRET_ACCESS_KEY", and these must hold the corresponding credentials.
    #[serde(rename = "awsSecretName")]
    pub aws_secret_name: String,
    /// Amazon S3 bucket name.
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// Optional. Amazon S3 subdir under bucket.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdir: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupSftp {
    /// The name of the K8s secret that holds SFTP credentials. The secret must contain the "Key" key, which is the SSH private key for connecting to the sftp server.
    #[serde(rename = "sftpSecretName")]
    pub sftp_secret_name: String,
    /// SFTP url
    pub sftp_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseBackupSwift {
    /// Swift service authentication URL.
    pub auth_url: String,
    /// Swift object store container for storing the backup files.
    pub container: String,
    /// Optional. Prefix (path) of backup files in the swift container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// The name of the K8s secret that holds Swift credentials. The secret must contain the keys "Key" and "User", and these must hold the corresponding credentials: service access key and service user name (pattern for the latter does not allow special characters &,<,>,")
    #[serde(rename = "swiftSecretName")]
    pub swift_secret_name: String,
}

/// Redis Enterprise Module: https://redislabs.com/redis-enterprise/modules/
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseModulesList {
    /// Module command line arguments e.g. VKEY_MAX_ENTITY_COUNT 30
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// The module's name e.g "ft" for redissearch
    pub name: String,
    /// Module's semantic version e.g "1.6.12"
    pub version: String,
}

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisEnterpriseDatabasePersistence {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "aofEverySecond")]
    AofEverySecond,
    #[serde(rename = "aofAlways")]
    AofAlways,
    #[serde(rename = "snapshotEvery1Hour")]
    SnapshotEvery1Hour,
    #[serde(rename = "snapshotEvery6Hour")]
    SnapshotEvery6Hour,
    #[serde(rename = "snapshotEvery12Hour")]
    SnapshotEvery12Hour,
}

/// Connection to Redis Enterprise Cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseRedisEnterpriseCluster {
    /// The name of the Redis Enterprise Cluster where the database should be stored.
    pub name: String,
}

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisEnterpriseDatabaseRedisVersion {
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "latest")]
    Latest,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseReplicaSources {
    /// Secret that defines the client certificate and key used by the syncer in the target database cluster. The secret must have 2 keys in its map: "cert" which is the PEM encoded certificate, and "key" which is the PEM encoded private key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientKeySecret")]
    pub client_key_secret: Option<String>,
    /// GZIP compression level (0-6) to use for replication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<i64>,
    /// The name of the resource from which the source database URI is derived. The type of resource must match the type specified in the ReplicaSourceType field.
    #[serde(rename = "replicaSourceName")]
    pub replica_source_name: String,
    /// The type of resource from which the source database URI is derived. If set to 'SECRET', the source database URI is derived from the secret named in the ReplicaSourceName field. The secret must have a key named 'uri' that defines the URI of the source database in the form of 'redis://...'. The type of secret (kubernetes, vault, ...) is determined by the secret mechanism used by the underlying REC object. If set to 'REDB', the source database URI is derived from the RedisEnterpriseDatabase resource named in the ReplicaSourceName field.
    #[serde(rename = "replicaSourceType")]
    pub replica_source_type: String,
    /// Secret that defines the server certificate used by the proxy in the source database cluster. The secret must have 1 key in its map: "cert" which is the PEM encoded certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertSecret")]
    pub server_cert_secret: Option<String>,
    /// TLS SNI name to use for the replication link.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSniName")]
    pub tls_sni_name: Option<String>,
}

/// Redis Enterprise Role and ACL Binding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseRolesPermissions {
    /// Acl Name of RolePermissionType
    pub acl: String,
    /// Role Name of RolePermissionType
    pub role: String,
    /// Type of Redis Enterprise Database Role Permission
    #[serde(rename = "type")]
    pub r#type: String,
}

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisEnterpriseDatabaseShardsPlacement {
    #[serde(rename = "dense")]
    Dense,
    #[serde(rename = "sparse")]
    Sparse,
}

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisEnterpriseDatabaseTlsMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "replica_ssl")]
    ReplicaSsl,
}

/// RedisEnterpriseDatabaseSpec defines the desired state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RedisEnterpriseDatabaseType {
    #[serde(rename = "redis")]
    Redis,
    #[serde(rename = "memcached")]
    Memcached,
}

/// RedisEnterpriseDatabaseStatus defines the observed state of RedisEnterpriseDatabase
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseStatus {
    /// Connection/ association to the Active-Active database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeActive")]
    pub active_active: Option<RedisEnterpriseDatabaseStatusActiveActive>,
    /// Information on the database's periodic backup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupInfo")]
    pub backup_info: Option<RedisEnterpriseDatabaseStatusBackupInfo>,
    /// Time when the database was created
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdTime")]
    pub created_time: Option<String>,
    /// Database UID provided by redis enterprise
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "databaseUID")]
    pub database_uid: Option<String>,
    /// Endpoints listed internally by the Redis Enterprise Cluster. Can be used to correlate a ReplicaSourceStatus entry.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "internalEndpoints")]
    pub internal_endpoints: Option<Vec<RedisEnterpriseDatabaseStatusInternalEndpoints>>,
    /// Status of the last action done by operator on this database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastActionStatus")]
    pub last_action_status: Option<String>,
    /// UID of the last action done by operator on this database
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastActionUid")]
    pub last_action_uid: Option<String>,
    /// Time when the database was last updated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
    /// The generation (built in update counter of K8s) of the REDB resource that was fully acted upon, meaning that all changes were handled and sent as an API call to the Redis Enterprise Cluster (REC). This field value should equal the current generation when the resource changes were handled. Note: the lastActionStatus field tracks actions handled asynchronously by the Redis Enterprise Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// The Redis Enterprise Cluster Object this Resource is associated with
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redisEnterpriseCluster")]
    pub redis_enterprise_cluster: Option<String>,
    /// ReplicaSource statuses
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaSourceStatuses")]
    pub replica_source_statuses: Option<Vec<RedisEnterpriseDatabaseStatusReplicaSourceStatuses>>,
    /// Aggregated statuses of shards
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shardStatuses")]
    pub shard_statuses: Option<BTreeMap<String, i64>>,
    /// Whether the desired specification is valid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specStatus")]
    pub spec_status: Option<String>,
    /// The status of the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Database compatibility version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Connection/ association to the Active-Active database.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseStatusActiveActive {
    /// The the corresponding Active-Active database name, Redis Enterprise Active Active Database custom resource name, this Resource is associated with. In case this resource is created manually at the active active database creation this field must be filled via the user, otherwise, the operator will assign this field automatically. Note: this feature is currently unsupported.
    pub name: String,
    /// The corresponding participating cluster name, Redis Enterprise Remote Cluster custom resource name, in the Active-Active database, In case this resource is created manually at the active active database creation this field must be filled via the user, otherwise, the operator will assign this field automatically. Note: this feature is currently unsupported.
    #[serde(rename = "participatingClusterName")]
    pub participating_cluster_name: String,
}

/// Information on the database's periodic backup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseStatusBackupInfo {
    /// Reason of last failed backup process
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupFailureReason")]
    pub backup_failure_reason: Option<String>,
    /// Backup history retention policy (number of days, 0 is forever)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupHistory")]
    pub backup_history: Option<i64>,
    /// Interval in seconds in which automatic backup will be initiated
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupInterval")]
    pub backup_interval: Option<i64>,
    /// Offset (in seconds) from round backup interval when automatic backup will be initiated (should be less than backup_interval)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupIntervalOffset")]
    pub backup_interval_offset: Option<i64>,
    /// Database scheduled periodic backup progress (percentage)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupProgressPercentage")]
    pub backup_progress_percentage: Option<i64>,
    /// Status of scheduled periodic backup process
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupStatus")]
    pub backup_status: Option<String>,
    /// Time of last successful backup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastBackupTime")]
    pub last_backup_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseStatusInternalEndpoints {
    /// Hostname assigned to the database
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Database port name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseDatabaseStatusReplicaSourceStatuses {
    /// The internal host name of the replica source database. Can be used as an identifier. See the internalEndpoints list on the REDB status.
    #[serde(rename = "endpointHost")]
    pub endpoint_host: String,
    /// Lag in millisec between source and destination (while synced).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lag: Option<i64>,
    /// Last error encountered when syncing from the source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastError")]
    pub last_error: Option<String>,
    /// Time when we last receive an update from the source.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdate")]
    pub last_update: Option<String>,
    /// The source’s RDB size to be transferred during the syncing phase.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rdbSize")]
    pub rdb_size: Option<i64>,
    /// Number of bytes transferred from the source’s RDB during the syncing phase.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rdbTransferred")]
    pub rdb_transferred: Option<i64>,
    /// Sync status of this source
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

