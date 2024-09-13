// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tungstenfabric/tf-operator/tf.tungsten.io/v1alpha1/analyticssnmp.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// AnalyticsSnmpSpec is the Spec for the Analytics SNMP API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tf.tungsten.io", version = "v1alpha1", kind = "AnalyticsSnmp", plural = "analyticssnmp")]
#[kube(namespaced)]
#[kube(status = "AnalyticsSnmpStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AnalyticsSnmpSpec {
    /// PodConfiguration is the common services struct.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonConfiguration")]
    pub common_configuration: Option<AnalyticsSnmpCommonConfiguration>,
    /// AnalyticsSnmpConfiguration is the Spec for the Analytics SNMP API.
    #[serde(rename = "serviceConfiguration")]
    pub service_configuration: AnalyticsSnmpServiceConfiguration,
}

/// PodConfiguration is the common services struct.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpCommonConfiguration {
    /// AuthParameters auth parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authParameters")]
    pub auth_parameters: Option<AnalyticsSnmpCommonConfigurationAuthParameters>,
    /// OS family
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<String>>,
    /// Kubernetes Cluster Configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<AnalyticsSnmpCommonConfigurationLogLevel>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// If specified, the pod's tolerations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<AnalyticsSnmpCommonConfigurationTolerations>>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpCommonConfigurationAuthParameters {
    /// AuthenticationMode auth mode
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authMode")]
    pub auth_mode: Option<AnalyticsSnmpCommonConfigurationAuthParametersAuthMode>,
    /// KeystoneAuthParameters keystone parameters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneAuthParameters")]
    pub keystone_auth_parameters: Option<AnalyticsSnmpCommonConfigurationAuthParametersKeystoneAuthParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keystoneSecretName")]
    pub keystone_secret_name: Option<String>,
}

/// AuthParameters auth parameters
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AnalyticsSnmpCommonConfigurationAuthParametersAuthMode {
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "keystone")]
    Keystone,
}

/// KeystoneAuthParameters keystone parameters
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpCommonConfigurationAuthParametersKeystoneAuthParameters {
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
pub enum AnalyticsSnmpCommonConfigurationLogLevel {
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
pub struct AnalyticsSnmpCommonConfigurationTolerations {
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

/// AnalyticsSnmpConfiguration is the Spec for the Analytics SNMP API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpServiceConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AnalyticsSnmpServiceConfigurationContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logFilePath")]
    pub log_file_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLocal")]
    pub log_local: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snmpCollectorFastScanFrequency")]
    pub snmp_collector_fast_scan_frequency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snmpCollectorIntrospectListenPort")]
    pub snmp_collector_introspect_listen_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snmpCollectorLogFileName")]
    pub snmp_collector_log_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snmpCollectorScanFrequency")]
    pub snmp_collector_scan_frequency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyIntrospectListenPort")]
    pub topology_introspect_listen_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyLogFileName")]
    pub topology_log_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologySnmpFrequency")]
    pub topology_snmp_frequency: Option<i64>,
}

/// Container defines name, image and command.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpServiceConfigurationContainers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// AnalyticsSnmpStatus is the Status for the Analytics SNMP API.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configChanged")]
    pub config_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<BTreeMap<String, AnalyticsSnmpStatusNodes>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AnalyticsSnmpStatusNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

