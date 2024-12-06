// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/strimzi/strimzi-kafka-operator/kafka.strimzi.io/v1beta2/kafkaconnectors.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// The specification of the Kafka Connector.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kafka.strimzi.io", version = "v1beta2", kind = "KafkaConnector", plural = "kafkaconnectors")]
#[kube(namespaced)]
#[kube(status = "KafkaConnectorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KafkaConnectorSpec {
    /// Configuration for altering offsets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alterOffsets")]
    pub alter_offsets: Option<KafkaConnectorAlterOffsets>,
    /// Automatic restart of connector and tasks configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoRestart")]
    pub auto_restart: Option<KafkaConnectorAutoRestart>,
    /// The Class for the Kafka Connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    /// The Kafka Connector configuration. The following properties cannot be set: name, connector.class, tasks.max.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, serde_json::Value>>,
    /// Configuration for listing offsets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "listOffsets")]
    pub list_offsets: Option<KafkaConnectorListOffsets>,
    /// Whether the connector should be paused. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pause: Option<bool>,
    /// The state the connector should be in. Defaults to running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<KafkaConnectorState>,
    /// The maximum number of tasks for the Kafka Connector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tasksMax")]
    pub tasks_max: Option<i64>,
}

/// Configuration for altering offsets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorAlterOffsets {
    /// Reference to the ConfigMap where the new offsets are stored.
    #[serde(rename = "fromConfigMap")]
    pub from_config_map: KafkaConnectorAlterOffsetsFromConfigMap,
}

/// Reference to the ConfigMap where the new offsets are stored.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorAlterOffsetsFromConfigMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Automatic restart of connector and tasks configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorAutoRestart {
    /// Whether automatic restart for failed connectors and tasks should be enabled or disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The maximum number of connector restarts that the operator will try. If the connector remains in a failed state after reaching this limit, it must be restarted manually by the user. Defaults to an unlimited number of restarts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRestarts")]
    pub max_restarts: Option<i64>,
}

/// Configuration for listing offsets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorListOffsets {
    /// Reference to the ConfigMap where the list of offsets will be written to.
    #[serde(rename = "toConfigMap")]
    pub to_config_map: KafkaConnectorListOffsetsToConfigMap,
}

/// Reference to the ConfigMap where the list of offsets will be written to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorListOffsetsToConfigMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// The specification of the Kafka Connector.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KafkaConnectorState {
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
}

/// The status of the Kafka Connector.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorStatus {
    /// The auto restart status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoRestart")]
    pub auto_restart: Option<KafkaConnectorStatusAutoRestart>,
    /// List of status conditions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The connector status, as reported by the Kafka Connect REST API.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectorStatus")]
    pub connector_status: Option<BTreeMap<String, serde_json::Value>>,
    /// The generation of the CRD that was last reconciled by the operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// The maximum number of tasks for the Kafka Connector.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tasksMax")]
    pub tasks_max: Option<i64>,
    /// The list of topics used by the Kafka Connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

/// The auto restart status.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KafkaConnectorStatusAutoRestart {
    /// The name of the connector being restarted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectorName")]
    pub connector_name: Option<String>,
    /// The number of times the connector or task is restarted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The last time the automatic restart was attempted. The required format is 'yyyy-MM-ddTHH:mm:ssZ' in the UTC time zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRestartTimestamp")]
    pub last_restart_timestamp: Option<String>,
}

