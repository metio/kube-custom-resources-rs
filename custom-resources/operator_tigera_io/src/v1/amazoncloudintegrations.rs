// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tigera/operator/operator.tigera.io/v1/amazoncloudintegrations.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// AmazonCloudIntegrationSpec defines the desired state of AmazonCloudIntegration
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "AmazonCloudIntegration", plural = "amazoncloudintegrations")]
#[kube(status = "AmazonCloudIntegrationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AmazonCloudIntegrationSpec {
    /// AWSRegion is the region in which your cluster is located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsRegion")]
    pub aws_region: Option<String>,
    /// DefaultPodMetadataAccess defines what the default behavior will be for accessing the AWS metadata service from a pod. Default: Denied
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultPodMetadataAccess")]
    pub default_pod_metadata_access: Option<AmazonCloudIntegrationDefaultPodMetadataAccess>,
    /// EnforcedSecurityGroupID is the ID of the Security Group which will be applied to all ENIs that are on a host that is also part of the Kubernetes cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enforcedSecurityGroupID")]
    pub enforced_security_group_id: Option<String>,
    /// NodeSecurityGroupIDs is a list of Security Group IDs that all nodes and masters will be in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSecurityGroupIDs")]
    pub node_security_group_i_ds: Option<Vec<String>>,
    /// PodSecurityGroupID is the ID of the Security Group which all pods should be placed in by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSecurityGroupID")]
    pub pod_security_group_id: Option<String>,
    /// SQSURL is the SQS URL needed to access the Simple Queue Service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sqsURL")]
    pub sqs_url: Option<String>,
    /// TrustEnforcedSecurityGroupID is the ID of the Security Group which will be applied to all ENIs in the VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trustEnforcedSecurityGroupID")]
    pub trust_enforced_security_group_id: Option<String>,
    /// VPCS is a list of VPC IDs to monitor for ENIs and Security Groups, only one is supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpcs: Option<Vec<String>>,
}

/// AmazonCloudIntegrationSpec defines the desired state of AmazonCloudIntegration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AmazonCloudIntegrationDefaultPodMetadataAccess {
    Allowed,
    Denied,
}

/// AmazonCloudIntegrationStatus defines the observed state of AmazonCloudIntegration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AmazonCloudIntegrationStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

