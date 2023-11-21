// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/nginxinc/kubernetes-ingress/appprotectdos.f5.com/v1beta1/apdoslogconfs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// APDosLogConfSpec defines the desired state of APDosLogConf
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "appprotectdos.f5.com", version = "v1beta1", kind = "APDosLogConf", plural = "apdoslogconfs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct APDosLogConfSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<APDosLogConfContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<APDosLogConfFilter>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APDosLogConfContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<APDosLogConfContentFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_message_size: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APDosLogConfContentFormat {
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "arcsight")]
    Arcsight,
    #[serde(rename = "user-defined")]
    UserDefined,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APDosLogConfFilter {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attack-signatures")]
    pub attack_signatures: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bad-actors")]
    pub bad_actors: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "traffic-mitigation-stats")]
    pub traffic_mitigation_stats: Option<APDosLogConfFilterTrafficMitigationStats>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum APDosLogConfFilterTrafficMitigationStats {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
}
