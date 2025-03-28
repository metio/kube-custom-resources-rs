// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tungstenfabric/tf-operator/tf.tungsten.io/v1alpha1/analyticsalarm.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// AnalyticsAlarmSpec is the Spec for the Analytics Alarm API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tf.tungsten.io", version = "v1alpha1", kind = "AnalyticsAlarm", plural = "analyticsalarm")]
#[kube(namespaced)]
#[kube(status = "AnalyticsAlarmStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AnalyticsAlarmSpec {
    /// PodConfiguration is the common services struct.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonConfiguration")]
    pub common_configuration: Option<AnalyticsAlarmCommonConfiguration>,
    /// AnalyticsAlarmConfiguration is the Spec for the Analytics Alarm API.
    #[serde(rename = "serviceConfiguration")]
    pub service_configuration: AnalyticsAlarmServiceConfiguration,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmCommonConfiguration {
    /// AuthParameters auth parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authParameters")]
    pub auth_parameters: Option<AnalyticsAlarmCommonConfigurationAuthParameters>,
    /// OS family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<String>>,
    /// Kubernetes Cluster Configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<AnalyticsAlarmCommonConfigurationLogLevel>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<AnalyticsAlarmCommonConfigurationTolerations>>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmCommonConfigurationAuthParameters {
    /// AuthenticationMode auth mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authMode")]
    pub auth_mode: Option<AnalyticsAlarmCommonConfigurationAuthParametersAuthMode>,
    /// KeystoneAuthParameters keystone parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthParameters")]
    pub keystone_auth_parameters: Option<AnalyticsAlarmCommonConfigurationAuthParametersKeystoneAuthParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneSecretName")]
    pub keystone_secret_name: Option<String>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AnalyticsAlarmCommonConfigurationAuthParametersAuthMode {
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "keystone")]
    Keystone,
}

/// KeystoneAuthParameters keystone parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmCommonConfigurationAuthParametersKeystoneAuthParameters {
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
pub enum AnalyticsAlarmCommonConfigurationLogLevel {
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
pub struct AnalyticsAlarmCommonConfigurationTolerations {
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

/// AnalyticsAlarmConfiguration is the Spec for the Analytics Alarm API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmServiceConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmgenIntrospectListenPort")]
    pub alarmgen_introspect_listen_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmgenLogFileName")]
    pub alarmgen_log_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmgenPartitions")]
    pub alarmgen_partitions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmgenRedisAggregateDbOffset")]
    pub alarmgen_redis_aggregate_db_offset: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AnalyticsAlarmServiceConfigurationContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFilePath")]
    pub log_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLocal")]
    pub log_local: Option<String>,
}

/// Container defines name, image and command.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmServiceConfigurationContainers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AnalyticsAlarmStatus is the Status for the Analytics Alarm API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configChanged")]
    pub config_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<BTreeMap<String, AnalyticsAlarmStatusNodes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsAlarmStatusNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

