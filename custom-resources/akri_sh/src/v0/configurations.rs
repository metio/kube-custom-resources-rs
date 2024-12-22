// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/project-akri/akri/akri.sh/v0/configurations.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "akri.sh", version = "v0", kind = "Configuration", plural = "configurations")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConfigurationSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerProperties")]
    pub broker_properties: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerSpec")]
    pub broker_spec: Option<ConfigurationBrokerSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationServiceSpec")]
    pub configuration_service_spec: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "discoveryHandler")]
    pub discovery_handler: Option<ConfigurationDiscoveryHandler>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceServiceSpec")]
    pub instance_service_spec: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationBrokerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerJobSpec")]
    pub broker_job_spec: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "brokerPodSpec")]
    pub broker_pod_spec: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscoveryHandler {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "discoveryDetails")]
    pub discovery_details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "discoveryProperties")]
    pub discovery_properties: Option<Vec<ConfigurationDiscoveryHandlerDiscoveryProperties>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscoveryHandlerDiscoveryProperties {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFromConfigMapKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFromSecretKeyRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFromConfigMapKeyRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigurationDiscoveryHandlerDiscoveryPropertiesValueFromSecretKeyRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

