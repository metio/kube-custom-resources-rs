// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/rds-controller/rds.services.k8s.aws/v1alpha1/dbparametergroups.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DBParameterGroupSpec defines the desired state of DBParameterGroup.
/// 
/// Contains the details of an Amazon RDS DB parameter group.
/// 
/// This data type is used as a response element in the DescribeDBParameterGroups
/// action.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rds.services.k8s.aws", version = "v1alpha1", kind = "DBParameterGroup", plural = "dbparametergroups")]
#[kube(namespaced)]
#[kube(status = "DBParameterGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DBParameterGroupSpec {
    /// The description for the DB parameter group.
    pub description: String,
    /// The DB parameter group family name. A DB parameter group can be associated
    /// with one and only one DB parameter group family, and can be applied only
    /// to a DB instance running a database engine and engine version compatible
    /// with that DB parameter group family.
    /// 
    /// To list all of the available parameter group families for a DB engine, use
    /// the following command:
    /// 
    /// aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily"
    /// --engine
    /// 
    /// For example, to list all of the available parameter group families for the
    /// MySQL DB engine, use the following command:
    /// 
    /// aws rds describe-db-engine-versions --query "DBEngineVersions[].DBParameterGroupFamily"
    /// --engine mysql
    /// 
    /// The output contains duplicates.
    /// 
    /// The following are the valid DB engine values:
    /// 
    ///    * aurora-mysql
    /// 
    ///    * aurora-postgresql
    /// 
    ///    * db2-ae
    /// 
    ///    * db2-se
    /// 
    ///    * mysql
    /// 
    ///    * oracle-ee
    /// 
    ///    * oracle-ee-cdb
    /// 
    ///    * oracle-se2
    /// 
    ///    * oracle-se2-cdb
    /// 
    ///    * postgres
    /// 
    ///    * sqlserver-ee
    /// 
    ///    * sqlserver-se
    /// 
    ///    * sqlserver-ex
    /// 
    ///    * sqlserver-web
    pub family: String,
    /// The name of the DB parameter group.
    /// 
    /// Constraints:
    /// 
    ///    * Must be 1 to 255 letters, numbers, or hyphens.
    /// 
    ///    * First character must be a letter
    /// 
    ///    * Can't end with a hyphen or contain two consecutive hyphens
    /// 
    /// This value is stored as a lowercase string.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parameterOverrides")]
    pub parameter_overrides: Option<BTreeMap<String, String>>,
    /// Tags to assign to the DB parameter group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DBParameterGroupTags>>,
}

/// Metadata assigned to an Amazon RDS resource consisting of a key-value pair.
/// 
/// For more information, see Tagging Amazon RDS resources (https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html)
/// in the Amazon RDS User Guide or Tagging Amazon Aurora and Amazon RDS resources
/// (https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/USER_Tagging.html)
/// in the Amazon Aurora User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBParameterGroupTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DBParameterGroupStatus defines the observed state of DBParameterGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBParameterGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DBParameterGroupStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A list of Parameter values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parameterOverrideStatuses")]
    pub parameter_override_statuses: Option<Vec<DBParameterGroupStatusParameterOverrideStatuses>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBParameterGroupStatusAckResourceMetadata {
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

/// This data type is used as a request parameter in the ModifyDBParameterGroup
/// and ResetDBParameterGroup actions.
/// 
/// This data type is used as a response element in the DescribeEngineDefaultParameters
/// and DescribeDBParameters actions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBParameterGroupStatusParameterOverrideStatuses {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedValues")]
    pub allowed_values: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyMethod")]
    pub apply_method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyType")]
    pub apply_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataType")]
    pub data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isModifiable")]
    pub is_modifiable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minimumEngineVersion")]
    pub minimum_engine_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parameterName")]
    pub parameter_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parameterValue")]
    pub parameter_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedEngineModes")]
    pub supported_engine_modes: Option<Vec<String>>,
}

