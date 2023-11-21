// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-policy-agent/gatekeeper/status.gatekeeper.sh/v1beta1/constrainttemplatepodstatuses.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2


use serde::{Serialize, Deserialize};

/// ConstraintTemplatePodStatusStatus defines the observed state of ConstraintTemplatePodStatus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplatePodStatusStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ConstraintTemplatePodStatusStatusErrors>>,
    /// Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// UID is a type that holds unique ID values, including UUIDs.  Because we don't ONLY use UUIDs, this is an alias to string.  Being a type captures intent and helps make sure that UIDs and names do not get conflated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateUID")]
    pub template_uid: Option<String>,
}

/// CreateCRDError represents a single error caught during parsing, compiling, etc.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplatePodStatusStatusErrors {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub message: String,
}
