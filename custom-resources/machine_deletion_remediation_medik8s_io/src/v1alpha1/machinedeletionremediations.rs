// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/medik8s/machine-deletion-remediation/machine-deletion-remediation.medik8s.io/v1alpha1/machinedeletionremediations.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// MachineDeletionRemediationSpec defines the desired state of MachineDeletionRemediation
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "machine-deletion-remediation.medik8s.io", version = "v1alpha1", kind = "MachineDeletionRemediation", plural = "machinedeletionremediations")]
#[kube(namespaced)]
#[kube(status = "MachineDeletionRemediationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MachineDeletionRemediationSpec {
}

/// MachineDeletionRemediationStatus defines the observed state of MachineDeletionRemediation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MachineDeletionRemediationStatus {
    /// Represents the observations of a MachineDeletionRemediation's current state.
    /// Known .status.conditions.type are: "Processing", "Succeeded" and "PermanentNodeDeletionExpected"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

