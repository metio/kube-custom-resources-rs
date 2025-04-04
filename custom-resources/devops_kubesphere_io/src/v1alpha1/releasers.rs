// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubesphere-sigs/ks-releaser-operator/devops.kubesphere.io/v1alpha1/releasers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ReleaserSpec defines the desired state of Releaser
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "devops.kubesphere.io", version = "v1alpha1", kind = "Releaser", plural = "releasers")]
#[kube(namespaced)]
#[kube(status = "ReleaserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ReleaserSpec {
    /// GitOps indicates to integrate with GitOps
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitOps")]
    pub git_ops: Option<ReleaserGitOps>,
    /// Phase is the stage of a release request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<ReleaserRepositories>>,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ReleaserSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// GitOps indicates to integrate with GitOps
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserGitOps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// Repository represents a git repository
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<ReleaserGitOpsRepository>,
    /// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<ReleaserGitOpsSecret>,
}

/// Repository represents a git repository
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserGitOpsRepository {
    /// Action indicates the action once the request phase to be ready
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Provider represents a git provider, such as: GitHub, Gitlab
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserGitOpsSecret {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Repository represents a git repository
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserRepositories {
    /// Action indicates the action once the request phase to be ready
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Provider represents a git provider, such as: GitHub, Gitlab
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserSecret {
    /// Name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// ReleaserStatus defines the observed state of Releaser
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ReleaserStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// Condition indicates the status of each git repositories
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReleaserStatusConditions {
    /// ConditionType is the type of condition
    #[serde(rename = "conditionType")]
    pub condition_type: String,
    pub message: String,
    /// ConditionStatus is the status of a condition
    pub status: String,
}

