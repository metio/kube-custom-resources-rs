// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/grafana/k6-operator/k6.io/v1alpha1/privateloadzones.yaml --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "k6.io", version = "v1alpha1", kind = "PrivateLoadZone", plural = "privateloadzones")]
#[kube(namespaced)]
#[kube(status = "PrivateLoadZoneStatus")]
#[kube(schema = "disabled")]
pub struct PrivateLoadZoneSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    pub resources: PrivateLoadZoneResources,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrivateLoadZoneResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PrivateLoadZoneResourcesClaims>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrivateLoadZoneResourcesClaims {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PrivateLoadZoneStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

