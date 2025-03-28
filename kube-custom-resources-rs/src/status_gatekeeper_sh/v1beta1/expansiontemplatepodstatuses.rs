// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-policy-agent/gatekeeper/status.gatekeeper.sh/v1beta1/expansiontemplatepodstatuses.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ExpansionTemplatePodStatusStatus defines the observed state of ExpansionTemplatePodStatus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplatePodStatusStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ExpansionTemplatePodStatusStatusErrors>>,
    /// Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we
    /// don't ONLY use UUIDs, this is an alias to string.  Being a type captures
    /// intent and helps make sure that UIDs and names do not get conflated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateUID")]
    pub template_uid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplatePodStatusStatusErrors {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

