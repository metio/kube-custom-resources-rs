// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tigera/operator/operator.tigera.io/v1/managementclusterconnections.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// ManagementClusterConnectionSpec defines the desired state of ManagementClusterConnection
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "ManagementClusterConnection", plural = "managementclusterconnections")]
#[kube(status = "ManagementClusterConnectionStatus")]
#[kube(schema = "disabled")]
pub struct ManagementClusterConnectionSpec {
    /// GuardianDeployment configures the guardian Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "guardianDeployment")]
    pub guardian_deployment: Option<ManagementClusterConnectionGuardianDeployment>,
    /// Specify where the managed cluster can reach the management cluster. Ex.: "10.128.0.10:30449". A managed cluster should be able to access this address. This field is used by managed clusters only.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementClusterAddr")]
    pub management_cluster_addr: Option<String>,
    /// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ManagementClusterConnectionTls>,
}

/// GuardianDeployment configures the guardian Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeployment {
    /// Spec is the specification of the guardian Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ManagementClusterConnectionGuardianDeploymentSpec>,
}

/// Spec is the specification of the guardian Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpec {
    /// Template describes the guardian Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ManagementClusterConnectionGuardianDeploymentSpecTemplate>,
}

/// Template describes the guardian Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplate {
    /// Spec is the guardian Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpec>,
}

/// Spec is the guardian Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpec {
    /// Containers is a list of guardian containers. If specified, this overrides the specified guardian Deployment containers. If omitted, the guardian Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of guardian init containers. If specified, this overrides the specified guardian Deployment init containers. If omitted, the guardian Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainers>>,
}

/// GuardianDeploymentContainer is a guardian Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the guardian Deployment container by name.
    pub name: ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory. If specified, this overrides the named guardian Deployment container's resources. If omitted, the guardian Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersResources>,
}

/// GuardianDeploymentContainer is a guardian Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "tigera-guardian")]
    TigeraGuardian,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory. If specified, this overrides the named guardian Deployment container's resources. If omitted, the guardian Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// GuardianDeploymentInitContainer is a guardian Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the guardian Deployment init container by name.
    pub name: String,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory. If specified, this overrides the named guardian Deployment init container's resources. If omitted, the guardian Deployment will use its default value for this init container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainersResources>,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory. If specified, this overrides the named guardian Deployment init container's resources. If omitted, the guardian Deployment will use its default value for this init container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims, that are used by this container. 
    ///  This is an alpha field and requires enabling the DynamicResourceAllocation feature gate. 
    ///  This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. Requests cannot exceed Limits. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionGuardianDeploymentSpecTemplateSpecInitContainersResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of the Pod where this field is used. It makes that resource available inside a container.
    pub name: String,
}

/// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionTls {
    /// CA indicates which verification method the tunnel client should use to verify the tunnel server's identity. 
    ///  When left blank or set to 'Tigera', the tunnel client will expect a self-signed cert to be included in the certificate bundle and will expect the cert to have a Common Name (CN) of 'voltron'. 
    ///  When set to 'Public', the tunnel client will use its installed system certs and will use the managementClusterAddr to verify the tunnel server's identity. 
    ///  Default: Tigera
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<ManagementClusterConnectionTlsCa>,
}

/// TLS provides options for configuring how Managed Clusters can establish an mTLS connection with the Management Cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagementClusterConnectionTlsCa {
    Tigera,
    Public,
}

/// ManagementClusterConnectionStatus defines the observed state of ManagementClusterConnection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ManagementClusterConnectionStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagementClusterConnectionStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: ManagementClusterConnectionStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagementClusterConnectionStatusConditionsStatus {
    True,
    False,
    Unknown,
}

