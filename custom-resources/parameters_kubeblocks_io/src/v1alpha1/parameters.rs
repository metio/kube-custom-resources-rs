// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/apecloud/kubeblocks/parameters.kubeblocks.io/v1alpha1/parameters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ParameterSpec defines the desired state of Parameter
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "parameters.kubeblocks.io", version = "v1alpha1", kind = "Parameter", plural = "parameters")]
#[kube(namespaced)]
#[kube(status = "ParameterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ParameterSpec {
    /// Specifies the name of the Cluster resource that this operation is targeting.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// Lists ComponentParametersSpec objects, each specifying a Component and its parameters and template updates.
    #[serde(rename = "componentParameters")]
    pub component_parameters: Vec<ParameterComponentParameters>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterComponentParameters {
    /// Specifies the name of the Component.
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// Specifies the user-defined configuration template or parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    /// Specifies the user-defined configuration template.
    /// 
    /// 
    /// When provided, the `importTemplateRef` overrides the default configuration template
    /// specified in `configSpec.templateRef`.
    /// This allows users to customize the configuration template according to their specific requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userConfigTemplates")]
    pub user_config_templates: Option<BTreeMap<String, ParameterComponentParametersUserConfigTemplates>>,
}

/// Specifies the user-defined configuration template.
/// 
/// 
/// When provided, the `importTemplateRef` overrides the default configuration template
/// specified in `configSpec.templateRef`.
/// This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterComponentParametersUserConfigTemplates {
    /// Specifies the namespace of the referenced configuration template ConfigMap object.
    /// An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ParameterComponentParametersUserConfigTemplatesPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateRef")]
    pub template_ref: Option<String>,
}

/// Specifies the user-defined configuration template.
/// 
/// 
/// When provided, the `importTemplateRef` overrides the default configuration template
/// specified in `configSpec.templateRef`.
/// This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterComponentParametersUserConfigTemplatesPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

/// ParameterStatus defines the observed state of Parameter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatus {
    /// Records the status of a reconfiguring operation if `opsRequest.spec.type` equals to "Reconfiguring".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "componentReconfiguringStatus")]
    pub component_reconfiguring_status: Option<Vec<ParameterStatusComponentReconfiguringStatus>>,
    /// Provides a description of any abnormal status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Represents the latest generation observed for this
    /// ClusterDefinition. It corresponds to the ConfigConstraint's generation, which is
    /// updated by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Indicates the current status of the configuration item.
    /// 
    /// 
    /// Possible values include "Creating", "Init", "Running", "Pending", "Merged", "MergeFailed", "FailedAndPause",
    /// "Upgrading", "Deleting", "FailedAndRetry", "Finished".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ParameterStatusPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatusComponentReconfiguringStatus {
    /// Specifies the name of the Component.
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// Describes the status of the component reconfiguring.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parameterStatus")]
    pub parameter_status: Option<Vec<ParameterStatusComponentReconfiguringStatusParameterStatus>>,
    /// Indicates the current status of the configuration item.
    /// 
    /// 
    /// Possible values include "Creating", "Init", "Running", "Pending", "Merged", "MergeFailed", "FailedAndPause",
    /// "Upgrading", "Deleting", "FailedAndRetry", "Finished".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ParameterStatusComponentReconfiguringStatusPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatusComponentReconfiguringStatusParameterStatus {
    /// Represents the last completed revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDoneRevision")]
    pub last_done_revision: Option<String>,
    /// Provides a description of any abnormal status. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Specifies the name of the configuration template. It is a required field and must be a string of maximum 63 characters.
    /// The name should only contain lowercase alphanumeric characters, hyphens, or periods. It should start and end with an alphanumeric character.
    pub name: String,
    /// Indicates the current status of the configuration item.
    /// 
    /// 
    /// Possible values include "Creating", "Init", "Running", "Pending", "Merged", "MergeFailed", "FailedAndPause",
    /// "Upgrading", "Deleting", "FailedAndRetry", "Finished".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<ParameterStatusComponentReconfiguringStatusParameterStatusPhase>,
    /// Provides detailed information about the execution of the configuration change. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileDetail")]
    pub reconcile_detail: Option<ParameterStatusComponentReconfiguringStatusParameterStatusReconcileDetail>,
    /// Represents the updated revision of the configuration item. This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateRevision")]
    pub update_revision: Option<String>,
    /// Contains the updated parameters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedParameters")]
    pub updated_parameters: Option<BTreeMap<String, ParameterStatusComponentReconfiguringStatusParameterStatusUpdatedParameters>>,
    /// Specifies the user-defined configuration template.
    /// 
    /// 
    /// When provided, the `importTemplateRef` overrides the default configuration template
    /// specified in `configSpec.templateRef`.
    /// This allows users to customize the configuration template according to their specific requirements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userConfigTemplates")]
    pub user_config_templates: Option<ParameterStatusComponentReconfiguringStatusParameterStatusUserConfigTemplates>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterStatusComponentReconfiguringStatusParameterStatusPhase {
    Creating,
    Init,
    Running,
    Pending,
    Merged,
    MergeFailed,
    FailedAndPause,
    Upgrading,
    Deleting,
    FailedAndRetry,
    Finished,
}

/// Provides detailed information about the execution of the configuration change. This field is optional.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatusComponentReconfiguringStatusParameterStatusReconcileDetail {
    /// Represents the current revision of the configuration item.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentRevision")]
    pub current_revision: Option<String>,
    /// Represents the error message generated when the execution of configuration changes fails.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errMessage")]
    pub err_message: Option<String>,
    /// Represents the outcome of the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "execResult")]
    pub exec_result: Option<String>,
    /// Represents the total number of pods that require execution of configuration changes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expectedCount")]
    pub expected_count: Option<i32>,
    /// Represents the policy applied during the most recent execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Represents the number of pods where configuration changes were successfully applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "succeedCount")]
    pub succeed_count: Option<i32>,
}

/// Contains the updated parameters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatusComponentReconfiguringStatusParameterStatusUpdatedParameters {
    /// Holds the configuration keys and values. This field is a workaround for issues found in kubebuilder and code-generator.
    /// Refer to https://github.com/kubernetes-sigs/kubebuilder/issues/528 and https://github.com/kubernetes/code-generator/issues/50 for more details.
    /// 
    /// 
    /// Represents the content of the configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Represents the updated parameters for a single configuration file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
}

/// Specifies the user-defined configuration template.
/// 
/// 
/// When provided, the `importTemplateRef` overrides the default configuration template
/// specified in `configSpec.templateRef`.
/// This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ParameterStatusComponentReconfiguringStatusParameterStatusUserConfigTemplates {
    /// Specifies the namespace of the referenced configuration template ConfigMap object.
    /// An empty namespace is equivalent to the "default" namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Defines the strategy for merging externally imported templates into component templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<ParameterStatusComponentReconfiguringStatusParameterStatusUserConfigTemplatesPolicy>,
    /// Specifies the name of the referenced configuration template ConfigMap object.
    #[serde(rename = "templateRef")]
    pub template_ref: String,
}

/// Specifies the user-defined configuration template.
/// 
/// 
/// When provided, the `importTemplateRef` overrides the default configuration template
/// specified in `configSpec.templateRef`.
/// This allows users to customize the configuration template according to their specific requirements.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterStatusComponentReconfiguringStatusParameterStatusUserConfigTemplatesPolicy {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterStatusComponentReconfiguringStatusPhase {
    Creating,
    Init,
    Running,
    Pending,
    Merged,
    MergeFailed,
    FailedAndPause,
    Upgrading,
    Deleting,
    FailedAndRetry,
    Finished,
}

/// ParameterStatus defines the observed state of Parameter
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ParameterStatusPhase {
    Creating,
    Init,
    Running,
    Pending,
    Merged,
    MergeFailed,
    FailedAndPause,
    Upgrading,
    Deleting,
    FailedAndRetry,
    Finished,
}

