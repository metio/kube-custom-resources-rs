// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/backends.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BackendSpec defines the desired state of Backend
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "Backend", plural = "backends")]
#[kube(namespaced)]
#[kube(status = "BackendStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackendSpec {
    /// Description is a human readable text of the backend
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mappingRules")]
    pub mapping_rules: Option<Vec<BackendMappingRules>>,
    /// Methods Map: system_name -> MethodSpec system_name attr is unique for all metrics AND methods In other words, if metric's system_name is A, there is no metric or method with system_name A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub methods: Option<BTreeMap<String, BackendMethods>>,
    /// Metrics Map: system_name -> MetricSpec system_name attr is unique for all metrics AND methods In other words, if metric's system_name is A, there is no metric or method with system_name A.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BTreeMap<String, BackendMetrics>>,
    /// Name is human readable name for the backend
    pub name: String,
    /// PrivateBaseURL Private Base URL of the API
    #[serde(rename = "privateBaseURL")]
    pub private_base_url: String,
    /// ProviderAccountRef references account provider credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountRef")]
    pub provider_account_ref: Option<BackendProviderAccountRef>,
    /// SystemName identifies uniquely the backend within the account provider Default value will be sanitized Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "systemName")]
    pub system_name: Option<String>,
}

/// MappingRuleSpec defines the desired state of Product's MappingRule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BackendMappingRules {
    #[serde(rename = "httpMethod")]
    pub http_method: BackendMappingRulesHttpMethod,
    pub increment: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<bool>,
    #[serde(rename = "metricMethodRef")]
    pub metric_method_ref: String,
    pub pattern: String,
}

/// MappingRuleSpec defines the desired state of Product's MappingRule
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackendMappingRulesHttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "CONNECT")]
    Connect,
}

/// Methods Map: system_name -> MethodSpec system_name attr is unique for all metrics AND methods In other words, if metric's system_name is A, there is no metric or method with system_name A.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackendMethods {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "friendlyName")]
    pub friendly_name: Option<String>,
}

/// Metrics Map: system_name -> MetricSpec system_name attr is unique for all metrics AND methods In other words, if metric's system_name is A, there is no metric or method with system_name A.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackendMetrics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "friendlyName")]
    pub friendly_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// ProviderAccountRef references account provider credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackendProviderAccountRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// BackendStatus defines the observed state of Backend
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackendStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendId")]
    pub backend_id: Option<i64>,
    /// Current state of the 3scale backend. Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration reflects the generation of the most recently observed Backend Spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// 3scale control plane host
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountHost")]
    pub provider_account_host: Option<String>,
}

