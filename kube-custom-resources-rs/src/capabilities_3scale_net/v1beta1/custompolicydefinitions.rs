// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/custompolicydefinitions.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// CustomPolicyDefinitionSpec defines the desired state of CustomPolicyDefinition
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "CustomPolicyDefinition", plural = "custompolicydefinitions")]
#[kube(namespaced)]
#[kube(status = "CustomPolicyDefinitionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CustomPolicyDefinitionSpec {
    /// Name is the name of the custom policy
    pub name: String,
    /// ProviderAccountRef references account provider credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountRef")]
    pub provider_account_ref: Option<CustomPolicyDefinitionProviderAccountRef>,
    /// Schema is the schema of the custom policy
    pub schema: CustomPolicyDefinitionSchema,
    /// Version is the version of the custom policy
    pub version: String,
}

/// ProviderAccountRef references account provider credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CustomPolicyDefinitionProviderAccountRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Schema is the schema of the custom policy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CustomPolicyDefinitionSchema {
    /// Schema the $schema keyword is used to declare that this is a JSON Schema.
    #[serde(rename = "$schema")]
    pub schema: String,
    /// Configuration defines the structural schema for the policy
    pub configuration: BTreeMap<String, serde_json::Value>,
    /// Description is an array of description messages for the custom policy schema
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// Name is the name of the custom policy schema
    pub name: String,
    /// Summary is the summary of the custom policy schema
    pub summary: String,
    /// Version is the version of the custom policy schema
    pub version: String,
}

/// CustomPolicyDefinitionStatus defines the observed state of CustomPolicyDefinition
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CustomPolicyDefinitionStatus {
    /// Current state of the custom policy resource.
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration reflects the generation of the most recently observed Backend Spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ID of the custom policy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyID")]
    pub policy_id: Option<i64>,
    /// ProviderAccountHost contains the 3scale account's provider URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountHost")]
    pub provider_account_host: Option<String>,
}

