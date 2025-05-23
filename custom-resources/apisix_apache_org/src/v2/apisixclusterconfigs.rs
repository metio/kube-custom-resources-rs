// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/apache/apisix-ingress-controller/apisix.apache.org/v2/apisixclusterconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apisix.apache.org", version = "v2", kind = "ApisixClusterConfig", plural = "apisixclusterconfigs")]
#[kube(status = "ApisixClusterConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ApisixClusterConfigSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<ApisixClusterConfigAdmin>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<ApisixClusterConfigMonitoring>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixClusterConfigAdmin {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "adminKey")]
    pub admin_key: Option<String>,
    #[serde(rename = "baseURL")]
    pub base_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixClusterConfigMonitoring {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prometheus: Option<ApisixClusterConfigMonitoringPrometheus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skywalking: Option<ApisixClusterConfigMonitoringSkywalking>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixClusterConfigMonitoringPrometheus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_name: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixClusterConfigMonitoringSkywalking {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sampleRatio")]
    pub sample_ratio: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixClusterConfigStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

