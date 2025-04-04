// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/open-policy-agent/gatekeeper/expansion.gatekeeper.sh/v1alpha1/expansiontemplate.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ExpansionTemplateSpec defines the desired state of ExpansionTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "expansion.gatekeeper.sh", version = "v1alpha1", kind = "ExpansionTemplate", plural = "expansiontemplate")]
#[kube(status = "ExpansionTemplateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ExpansionTemplateSpec {
    /// ApplyTo lists the specific groups, versions and kinds of generator resources
    /// which will be expanded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyTo")]
    pub apply_to: Option<Vec<ExpansionTemplateApplyTo>>,
    /// EnforcementAction specifies the enforcement action to be used for resources
    /// matching the ExpansionTemplate. Specifying an empty value will use the
    /// enforcement action specified by the Constraint in violation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enforcementAction")]
    pub enforcement_action: Option<String>,
    /// GeneratedGVK specifies the GVK of the resources which the generator
    /// resource creates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generatedGVK")]
    pub generated_gvk: Option<ExpansionTemplateGeneratedGvk>,
    /// TemplateSource specifies the source field on the generator resource to
    /// use as the base for expanded resource. For Pod-creating generators, this
    /// is usually spec.template
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateSource")]
    pub template_source: Option<String>,
}

/// ApplyTo determines what GVKs items the mutation should apply to.
/// Globs are not allowed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplateApplyTo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

/// GeneratedGVK specifies the GVK of the resources which the generator
/// resource creates.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplateGeneratedGvk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// ExpansionTemplateStatus defines the observed state of ExpansionTemplate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplateStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "byPod")]
    pub by_pod: Option<Vec<ExpansionTemplateStatusByPod>>,
}

/// ExpansionTemplatePodStatusStatus defines the observed state of ExpansionTemplatePodStatus.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExpansionTemplateStatusByPod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ExpansionTemplateStatusByPodErrors>>,
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
pub struct ExpansionTemplateStatusByPodErrors {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

