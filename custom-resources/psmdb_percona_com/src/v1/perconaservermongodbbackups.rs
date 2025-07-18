// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/percona/percona-server-mongodb-operator/psmdb.percona.com/v1/perconaservermongodbbackups.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "psmdb.percona.com", version = "v1", kind = "PerconaServerMongoDBBackup", plural = "perconaservermongodbbackups")]
#[kube(namespaced)]
#[kube(status = "PerconaServerMongoDBBackupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PerconaServerMongoDBBackupSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionLevel")]
    pub compression_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionType")]
    pub compression_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startingDeadlineSeconds")]
    pub starting_deadline_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<PerconaServerMongoDBBackupType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PerconaServerMongoDBBackupType {
    #[serde(rename = "logical")]
    Logical,
    #[serde(rename = "physical")]
    Physical,
    #[serde(rename = "incremental")]
    Incremental,
    #[serde(rename = "incremental-base")]
    IncrementalBase,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<PerconaServerMongoDBBackupStatusAzure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<PerconaServerMongoDBBackupStatusFilesystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcs: Option<PerconaServerMongoDBBackupStatusGcs>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransition")]
    pub last_transition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastWriteAt")]
    pub last_write_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestRestorableTime")]
    pub latest_restorable_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmName")]
    pub pbm_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmPod")]
    pub pbm_pod: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pbmPods")]
    pub pbm_pods: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replsetNames")]
    pub replset_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3: Option<PerconaServerMongoDBBackupStatusS3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageName")]
    pub storage_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusAzure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusFilesystem {
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusGcs {
    pub bucket: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chunkSize")]
    pub chunk_size: Option<i64>,
    #[serde(rename = "credentialsSecret")]
    pub credentials_secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retryer: Option<PerconaServerMongoDBBackupStatusGcsRetryer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusGcsRetryer {
    #[serde(rename = "backoffInitial")]
    pub backoff_initial: i64,
    #[serde(rename = "backoffMax")]
    pub backoff_max: i64,
    #[serde(rename = "backoffMultiplier")]
    pub backoff_multiplier: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusS3 {
    pub bucket: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecret")]
    pub credentials_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "debugLogLevels")]
    pub debug_log_levels: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointUrl")]
    pub endpoint_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forcePathStyle")]
    pub force_path_style: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "insecureSkipTLSVerify")]
    pub insecure_skip_tls_verify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUploadParts")]
    pub max_upload_parts: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retryer: Option<PerconaServerMongoDBBackupStatusS3Retryer>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverSideEncryption")]
    pub server_side_encryption: Option<PerconaServerMongoDBBackupStatusS3ServerSideEncryption>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClass")]
    pub storage_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uploadPartSize")]
    pub upload_part_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusS3Retryer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetryDelay")]
    pub max_retry_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minRetryDelay")]
    pub min_retry_delay: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numMaxRetries")]
    pub num_max_retries: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PerconaServerMongoDBBackupStatusS3ServerSideEncryption {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseAlgorithm")]
    pub sse_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseCustomerAlgorithm")]
    pub sse_customer_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sseCustomerKey")]
    pub sse_customer_key: Option<String>,
}

