// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/bpfman/bpfman/bpfman.io/v1alpha1/bpfprograms.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BpfProgramSpec defines the desired state of BpfProgram
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "bpfman.io", version = "v1alpha1", kind = "BpfProgram", plural = "bpfprograms")]
#[kube(status = "BpfProgramStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BpfProgramSpec {
    /// Type specifies the bpf program type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// BpfProgramStatus defines the observed state of BpfProgram TODO Make these a fixed set of metav1.Condition.types and metav1.Condition.reasons
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfProgramStatus {
    /// Conditions houses the updates regarding the actual implementation of the bpf program on the node Known .status.conditions.type are: "Available", "Progressing", and "Degraded"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

