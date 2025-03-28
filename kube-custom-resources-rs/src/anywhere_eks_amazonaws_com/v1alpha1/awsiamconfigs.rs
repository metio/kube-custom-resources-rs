// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/awsiamconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AWSIamConfigSpec defines the desired state of AWSIamConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "AWSIamConfig", plural = "awsiamconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AWSIamConfigSpec {
    /// AWSRegion defines a region in an AWS partition
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    /// BackendMode defines multiple backends for aws-iam-authenticator server
    /// The server searches for mappings in order
    #[serde(rename = "backendMode")]
    pub backend_mode: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapRoles")]
    pub map_roles: Option<Vec<AWSIamConfigMapRoles>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapUsers")]
    pub map_users: Option<Vec<AWSIamConfigMapUsers>>,
    /// Partition defines the AWS partition on which the IAM roles exist
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
}

/// MapRoles defines IAM role to a username and set of groups mapping using EKSConfigMap BackendMode.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWSIamConfigMapRoles {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "roleARN")]
    pub role_arn: String,
    pub username: String,
}

/// MapUsers defines IAM role to a username and set of groups mapping using EKSConfigMap BackendMode.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWSIamConfigMapUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "userARN")]
    pub user_arn: String,
    pub username: String,
}

/// AWSIamConfigStatus defines the observed state of AWSIamConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AWSIamConfigStatus {
}

