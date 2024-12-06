// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tigera/operator/operator.tigera.io/v1/policyrecommendations.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// PolicyRecommendationSpec defines configuration for the Calico Enterprise Policy Recommendation
/// service.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "PolicyRecommendation", plural = "policyrecommendations")]
#[kube(status = "PolicyRecommendationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PolicyRecommendationSpec {
    /// PolicyRecommendation configures the PolicyRecommendation Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyRecommendationDeployment")]
    pub policy_recommendation_deployment: Option<PolicyRecommendationPolicyRecommendationDeployment>,
}

/// PolicyRecommendation configures the PolicyRecommendation Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeployment {
    /// Spec is the specification of the PolicyRecommendation Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PolicyRecommendationPolicyRecommendationDeploymentSpec>,
}

/// Spec is the specification of the PolicyRecommendation Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpec {
    /// Template describes the PolicyRecommendation Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplate>,
}

/// Template describes the PolicyRecommendation Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplate {
    /// Spec is the PolicyRecommendation Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpec>,
}

/// Spec is the PolicyRecommendation Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpec {
    /// Containers is a list of PolicyRecommendation containers.
    /// If specified, this overrides the specified PolicyRecommendation Deployment containers.
    /// If omitted, the PolicyRecommendation Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of PolicyRecommendation init containers.
    /// If specified, this overrides the specified PolicyRecommendation Deployment init containers.
    /// If omitted, the PolicyRecommendation Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainers>>,
}

/// PolicyRecommendationDeploymentContainer is a PolicyRecommendation Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the PolicyRecommendation Deployment container by name.
    /// Supported values are: policy-recommendation-controller
    pub name: PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named PolicyRecommendation Deployment container's resources.
    /// If omitted, the PolicyRecommendation Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersResources>,
}

/// PolicyRecommendationDeploymentContainer is a PolicyRecommendation Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "policy-recommendation-controller")]
    PolicyRecommendationController,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named PolicyRecommendation Deployment container's resources.
/// If omitted, the PolicyRecommendation Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// PolicyRecommendationDeploymentInitContainer is a PolicyRecommendation Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the PolicyRecommendation Deployment init container by name.
    pub name: PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named PolicyRecommendation Deployment init container's resources.
    /// If omitted, the PolicyRecommendation Deployment will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersResources>,
}

/// PolicyRecommendationDeploymentInitContainer is a PolicyRecommendation Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersName {
    #[serde(rename = "policy-recommendation-tls-key-cert-provisioner")]
    PolicyRecommendationTlsKeyCertProvisioner,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named PolicyRecommendation Deployment init container's resources.
/// If omitted, the PolicyRecommendation Deployment will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationPolicyRecommendationDeploymentSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
}

/// PolicyRecommendationStatus defines the observed state of Tigera policy recommendation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PolicyRecommendationStatus {
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

