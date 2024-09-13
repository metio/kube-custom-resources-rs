// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/RedisLabs/redis-enterprise-k8s-docs/app.redislabs.com/v1alpha1/redisenterpriseremoteclusters.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "app.redislabs.com", version = "v1alpha1", kind = "RedisEnterpriseRemoteCluster", plural = "redisenterpriseremoteclusters")]
#[kube(namespaced)]
#[kube(status = "RedisEnterpriseRemoteClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RedisEnterpriseRemoteClusterSpec {
    /// The URL of the cluster, will be used for the active-active database URL.
    #[serde(rename = "apiFqdnUrl")]
    pub api_fqdn_url: String,
    /// The database URL suffix, will be used for the active-active database replication endpoint and replication endpoint SNI.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbFqdnSuffix")]
    pub db_fqdn_suffix: Option<String>,
    /// The name of the REC that the RERC is pointing at
    #[serde(rename = "recName")]
    pub rec_name: String,
    /// The namespace of the REC that the RERC is pointing at
    #[serde(rename = "recNamespace")]
    pub rec_namespace: String,
    /// The name of the secret containing cluster credentials. Must be of the following format: "redis-enterprise-<RERC name>"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RedisEnterpriseRemoteClusterStatus {
    /// Indicates whether this object represents a local or a remote cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    /// The most recent generation observed for this RERC. It corresponds to the RERC's generation, which is updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Whether the desired specification is valid.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specStatus")]
    pub spec_status: Option<String>,
    /// The status of the remote cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

