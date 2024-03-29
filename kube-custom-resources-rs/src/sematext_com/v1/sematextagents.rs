// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/sematext/sematext-operator/sematext.com/v1/sematextagents.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// Your account data
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sematext.com", version = "v1", kind = "SematextAgent", plural = "sematextagents")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct SematextAgentSpec {
    /// The Infra App token from your SC account
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infraToken")]
    pub infra_token: Option<String>,
    /// The region where your account data is hosted. Can be EU or US
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

