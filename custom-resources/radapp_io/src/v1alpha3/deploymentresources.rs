// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/radius-project/radius/radapp.io/v1alpha3/deploymentresources.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DeploymentResourceSpec defines the desired state of a DeploymentResource resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "radapp.io", version = "v1alpha3", kind = "DeploymentResource", plural = "deploymentresources")]
#[kube(namespaced)]
#[kube(status = "DeploymentResourceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeploymentResourceSpec {
    /// Id is the resource id of the Radius resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// DeploymentResourceStatus defines the observed state of a DeploymentResource resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeploymentResourceStatus {
    /// Id is the resource id of the Radius resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ObservedGeneration is the most recent generation observed for this DeploymentResource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Operation tracks the status of an in-progress provisioning operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<DeploymentResourceStatusOperation>,
    /// Phrase indicates the current status of the Deployment Resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
}

/// Operation tracks the status of an in-progress provisioning operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeploymentResourceStatusOperation {
    /// OperationKind describes the type of operation being performed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationKind")]
    pub operation_kind: Option<String>,
    /// ResumeToken is a token that can be used to resume an in-progress provisioning operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resumeToken")]
    pub resume_token: Option<String>,
}

