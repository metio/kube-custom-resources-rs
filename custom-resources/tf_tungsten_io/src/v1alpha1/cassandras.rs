// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tungstenfabric/tf-operator/tf.tungsten.io/v1alpha1/cassandras.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// CassandraSpec is the Spec for the cassandras API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tf.tungsten.io", version = "v1alpha1", kind = "Cassandra", plural = "cassandras")]
#[kube(namespaced)]
#[kube(status = "CassandraStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CassandraSpec {
    /// PodConfiguration is the common services struct.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonConfiguration")]
    pub common_configuration: Option<CassandraCommonConfiguration>,
    /// CassandraConfiguration is the Spec for the cassandras API.
    #[serde(rename = "serviceConfiguration")]
    pub service_configuration: CassandraServiceConfiguration,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraCommonConfiguration {
    /// AuthParameters auth parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authParameters")]
    pub auth_parameters: Option<CassandraCommonConfigurationAuthParameters>,
    /// OS family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<String>>,
    /// Kubernetes Cluster Configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<CassandraCommonConfigurationLogLevel>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<CassandraCommonConfigurationTolerations>>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraCommonConfigurationAuthParameters {
    /// AuthenticationMode auth mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authMode")]
    pub auth_mode: Option<CassandraCommonConfigurationAuthParametersAuthMode>,
    /// KeystoneAuthParameters keystone parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthParameters")]
    pub keystone_auth_parameters: Option<CassandraCommonConfigurationAuthParametersKeystoneAuthParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneSecretName")]
    pub keystone_secret_name: Option<String>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CassandraCommonConfigurationAuthParametersAuthMode {
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "keystone")]
    Keystone,
}

/// KeystoneAuthParameters keystone parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraCommonConfigurationAuthParametersKeystoneAuthParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPassword")]
    pub admin_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminPort")]
    pub admin_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminTenant")]
    pub admin_tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminUsername")]
    pub admin_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authProtocol")]
    pub auth_protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectDomainName")]
    pub project_domain_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userDomainName")]
    pub user_domain_name: Option<String>,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CassandraCommonConfigurationLogLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "none")]
    None,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraCommonConfigurationTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// CassandraConfiguration is the Spec for the cassandras API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraServiceConfiguration {
    /// CassandraConfigParameters defines additional parameters for Cassandra confgiuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cassandraParameters")]
    pub cassandra_parameters: Option<CassandraServiceConfigurationCassandraParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<CassandraServiceConfigurationContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cqlPort")]
    pub cql_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jmxLocalPort")]
    pub jmx_local_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listenAddress")]
    pub listen_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxHeapSize")]
    pub max_heap_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minHeapSize")]
    pub min_heap_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumDiskGB")]
    pub minimum_disk_gb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reaperAdmPort")]
    pub reaper_adm_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reaperAppPort")]
    pub reaper_app_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reaperEnabled")]
    pub reaper_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sslStoragePort")]
    pub ssl_storage_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startRPC")]
    pub start_rpc: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storagePort")]
    pub storage_port: Option<i64>,
}

/// CassandraConfigParameters defines additional parameters for Cassandra confgiuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraServiceConfigurationCassandraParameters {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compactionThroughputMbPerSec")]
    pub compaction_throughput_mb_per_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentCompactors")]
    pub concurrent_compactors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentCounterWrites")]
    pub concurrent_counter_writes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentMaterializedViewWrites")]
    pub concurrent_materialized_view_writes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentReads")]
    pub concurrent_reads: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrentWrites")]
    pub concurrent_writes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memtableAllocationType")]
    pub memtable_allocation_type: Option<CassandraServiceConfigurationCassandraParametersMemtableAllocationType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memtableFlushWriters")]
    pub memtable_flush_writers: Option<i64>,
}

/// CassandraConfigParameters defines additional parameters for Cassandra confgiuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CassandraServiceConfigurationCassandraParametersMemtableAllocationType {
    #[serde(rename = "heap_buffers")]
    HeapBuffers,
    #[serde(rename = "offheap_buffers")]
    OffheapBuffers,
    #[serde(rename = "offheap_objects")]
    OffheapObjects,
}

/// Container defines name, image and command.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraServiceConfigurationContainers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// CassandraStatus defines the status of the cassandra object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configChanged")]
    pub config_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<BTreeMap<String, CassandraStatusNodes>>,
    /// CassandraStatusPorts defines the status of the ports of the cassandra object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<CassandraStatusPorts>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraStatusNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// CassandraStatusPorts defines the status of the ports of the cassandra object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CassandraStatusPorts {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cqlPort")]
    pub cql_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jmxPort")]
    pub jmx_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

