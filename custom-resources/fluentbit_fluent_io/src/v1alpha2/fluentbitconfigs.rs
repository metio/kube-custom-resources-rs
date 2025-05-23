// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/fluent/fluent-operator/fluentbit.fluent.io/v1alpha2/fluentbitconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// NamespacedFluentBitCfgSpec defines the desired state of FluentBit
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "fluentbit.fluent.io", version = "v1alpha2", kind = "FluentBitConfig", plural = "fluentbitconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FluentBitConfigSpec {
    /// Select cluster level multiline parser config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterMultilineParserSelector")]
    pub cluster_multiline_parser_selector: Option<FluentBitConfigClusterMultilineParserSelector>,
    /// Select cluster level parser config
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterParserSelector")]
    pub cluster_parser_selector: Option<FluentBitConfigClusterParserSelector>,
    /// Select filter plugins
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterSelector")]
    pub filter_selector: Option<FluentBitConfigFilterSelector>,
    /// Select multiline parser plugins
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multilineParserSelector")]
    pub multiline_parser_selector: Option<FluentBitConfigMultilineParserSelector>,
    /// Select output plugins
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputSelector")]
    pub output_selector: Option<FluentBitConfigOutputSelector>,
    /// Select parser plugins
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parserSelector")]
    pub parser_selector: Option<FluentBitConfigParserSelector>,
    /// Service defines the global behaviour of the Fluent Bit engine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<FluentBitConfigService>,
}

/// Select cluster level multiline parser config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigClusterMultilineParserSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigClusterMultilineParserSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigClusterMultilineParserSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Select cluster level parser config
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigClusterParserSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigClusterParserSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigClusterParserSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Select filter plugins
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigFilterSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigFilterSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigFilterSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Select multiline parser plugins
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigMultilineParserSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigMultilineParserSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigMultilineParserSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Select output plugins
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigOutputSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigOutputSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigOutputSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Select parser plugins
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigParserSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<FluentBitConfigParserSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigParserSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Service defines the global behaviour of the Fluent Bit engine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigService {
    /// If true go to background on start
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daemon: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emitterMemBufLimit")]
    pub emitter_mem_buf_limit: Option<String>,
    /// Per-namespace re-emitter configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emitterName")]
    pub emitter_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "emitterStorageType")]
    pub emitter_storage_type: Option<String>,
    /// Enable input/output tracing on debug images, controlled more granualry via the http API
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableChunkTrace")]
    pub enable_chunk_trace: Option<bool>,
    /// Interval to flush output
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flushSeconds")]
    pub flush_seconds: Option<f64>,
    /// Wait time on exit
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "graceSeconds")]
    pub grace_seconds: Option<i64>,
    /// the error count to meet the unhealthy requirement, this is a sum for all output plugins in a defined HC_Period, example for output error: [2022/02/16 10:44:10] [ warn] [engine] failed to flush chunk '1-1645008245.491540684.flb', retry in 7 seconds: task_id=0, input=forward.1 > output=cloudwatch_logs.3 (out_id=3)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hcErrorsCount")]
    pub hc_errors_count: Option<i64>,
    /// The time period by second to count the error and retry failure data point
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hcPeriod")]
    pub hc_period: Option<i64>,
    /// the retry failure count to meet the unhealthy requirement, this is a sum for all output plugins in a defined HC_Period, example for retry failure: [2022/02/16 20:11:36] [ warn] [engine] chunk '1-1645042288.260516436.flb' cannot be retried: task_id=0, input=tcp.3 > output=cloudwatch_logs.1
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hcRetryFailureCount")]
    pub hc_retry_failure_count: Option<i64>,
    /// enable Health check feature at http://127.0.0.1:2020/api/v1/health Note: Enabling this will not automatically configure kubernetes to use fluentbit's healthcheck endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<bool>,
    /// If true enable reloading via HTTP
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hotReload")]
    pub hot_reload: Option<bool>,
    /// Address to listen
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpListen")]
    pub http_listen: Option<String>,
    /// Port to listen
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpPort")]
    pub http_port: Option<i32>,
    /// If true enable statistics HTTP server
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpServer")]
    pub http_server: Option<bool>,
    /// File to log diagnostic output
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFile")]
    pub log_file: Option<String>,
    /// Diagnostic level (error/warning/info/debug/trace)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<FluentBitConfigServiceLogLevel>,
    /// Optional 'parsers' config file (can be multiple)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parsersFile")]
    pub parsers_file: Option<String>,
    /// backward compatible
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parsersFiles")]
    pub parsers_files: Option<Vec<String>>,
    /// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<FluentBitConfigServiceStorage>,
}

/// Service defines the global behaviour of the Fluent Bit engine.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FluentBitConfigServiceLogLevel {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}

/// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FluentBitConfigServiceStorage {
    /// This option configure a hint of maximum value of memory to use when processing these records
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backlogMemLimit")]
    pub backlog_mem_limit: Option<String>,
    /// Enable the data integrity check when writing and reading data from the filesystem
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<FluentBitConfigServiceStorageChecksum>,
    /// When enabled, irrecoverable chunks will be deleted during runtime, and any other irrecoverable chunk located in the configured storage path directory will be deleted when Fluent-Bit starts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deleteIrrecoverableChunks")]
    pub delete_irrecoverable_chunks: Option<FluentBitConfigServiceStorageDeleteIrrecoverableChunks>,
    /// If the input plugin has enabled filesystem storage type, this property sets the maximum number of Chunks that can be up in memory
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxChunksUp")]
    pub max_chunks_up: Option<i64>,
    /// If http_server option has been enabled in the Service section, this option registers a new endpoint where internal metrics of the storage layer can be consumed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FluentBitConfigServiceStorageMetrics>,
    /// Select an optional location in the file system to store streams and chunks of data/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Configure the synchronization mode used to store the data into the file system
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<FluentBitConfigServiceStorageSync>,
}

/// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FluentBitConfigServiceStorageChecksum {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FluentBitConfigServiceStorageDeleteIrrecoverableChunks {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FluentBitConfigServiceStorageMetrics {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

/// Configure a global environment for the storage layer in Service. It is recommended to configure the volume and volumeMount separately for this storage. The hostPath type should be used for that Volume in Fluentbit daemon set.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FluentBitConfigServiceStorageSync {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "full")]
    Full,
}

