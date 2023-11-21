// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/open-policy-agent/gatekeeper/status.gatekeeper.sh/v1beta1/constraintpodstatuses.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2


use serde::{Serialize, Deserialize};

/// ConstraintPodStatusStatus defines the observed state of ConstraintPodStatus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintPodStatusStatus {
    /// Storing the constraint UID allows us to detect drift, such as when a constraint has been recreated after its CRD was deleted out from under it, interrupting the watch
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintUID")]
    pub constraint_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ConstraintPodStatusStatusErrors>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
}

/// Error represents a single error caught while adding a constraint to OPA.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintPodStatusStatusErrors {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub message: String,
}
