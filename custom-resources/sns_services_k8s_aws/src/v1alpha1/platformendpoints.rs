// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sns-controller/sns.services.k8s.aws/v1alpha1/platformendpoints.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// PlatformEndpointSpec defines the desired state of PlatformEndpoint.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sns.services.k8s.aws", version = "v1alpha1", kind = "PlatformEndpoint", plural = "platformendpoints")]
#[kube(namespaced)]
#[kube(status = "PlatformEndpointStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PlatformEndpointSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customUserData")]
    pub custom_user_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// PlatformApplicationArn returned from CreatePlatformApplication is used to
    /// create a an endpoint.
    #[serde(rename = "platformApplicationARN")]
    pub platform_application_arn: String,
    pub token: String,
}

/// PlatformEndpointStatus defines the observed state of PlatformEndpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlatformEndpointStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<PlatformEndpointStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// EndpointArn returned from CreateEndpoint action.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointARN")]
    pub endpoint_arn: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlatformEndpointStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

