// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubecost/cluster-turndown/kubecost.com/v1alpha1/turndownschedules.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kubecost.com", version = "v1alpha1", kind = "TurndownSchedule", plural = "turndownschedules")]
#[kube(status = "TurndownScheduleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TurndownScheduleSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat: Option<TurndownScheduleRepeat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TurndownScheduleRepeat {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TurndownScheduleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextScaleDownTime")]
    pub next_scale_down_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextScaleUpTime")]
    pub next_scale_up_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleDownId")]
    pub scale_down_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleDownMetadata")]
    pub scale_down_metadata: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleUpID")]
    pub scale_up_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleUpMetadata")]
    pub scale_up_metadata: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

