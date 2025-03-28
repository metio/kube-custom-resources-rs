// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/emrcontainers-controller/emrcontainers.services.k8s.aws/v1alpha1/jobruns.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// JobRunSpec defines the desired state of JobRun.
/// 
/// This entity describes a job run. A job run is a unit of work, such as a Spark
/// jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on
/// EKS.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "emrcontainers.services.k8s.aws", version = "v1alpha1", kind = "JobRun", plural = "jobruns")]
#[kube(namespaced)]
#[kube(status = "JobRunStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct JobRunSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configurationOverrides")]
    pub configuration_overrides: Option<String>,
    /// The execution role ARN for the job run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionRoleARN")]
    pub execution_role_arn: Option<String>,
    /// The job driver for the job run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobDriver")]
    pub job_driver: Option<JobRunJobDriver>,
    /// The name of the job run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Amazon EMR release version to use for the job run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "releaseLabel")]
    pub release_label: Option<String>,
    /// The tags assigned to job runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    /// The virtual cluster ID for which the job run request is submitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualClusterID")]
    pub virtual_cluster_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualClusterRef")]
    pub virtual_cluster_ref: Option<JobRunVirtualClusterRef>,
}

/// The job driver for the job run.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunJobDriver {
    /// The information about job driver for Spark submit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sparkSubmitJobDriver")]
    pub spark_submit_job_driver: Option<JobRunJobDriverSparkSubmitJobDriver>,
}

/// The information about job driver for Spark submit.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunJobDriverSparkSubmitJobDriver {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPoint")]
    pub entry_point: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPointArguments")]
    pub entry_point_arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sparkSubmitParameters")]
    pub spark_submit_parameters: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunVirtualClusterRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<JobRunVirtualClusterRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunVirtualClusterRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// JobRunStatus defines the observed state of JobRun
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<JobRunStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// This output displays the started job run ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The state of the job run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobRunStatusAckResourceMetadata {
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

