// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/monitoringschedules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// MonitoringScheduleSpec defines the desired state of MonitoringSchedule.
/// 
/// A schedule for a model monitoring job. For information about model monitor,
/// see Amazon SageMaker Model Monitor (https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor.html).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "MonitoringSchedule", plural = "monitoringschedules")]
#[kube(namespaced)]
#[kube(status = "MonitoringScheduleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MonitoringScheduleSpec {
    /// The configuration object that specifies the monitoring schedule and defines
    /// the monitoring job.
    #[serde(rename = "monitoringScheduleConfig")]
    pub monitoring_schedule_config: MonitoringScheduleMonitoringScheduleConfig,
    /// The name of the monitoring schedule. The name must be unique within an Amazon
    /// Web Services Region within an Amazon Web Services account.
    #[serde(rename = "monitoringScheduleName")]
    pub monitoring_schedule_name: String,
    /// (Optional) An array of key-value pairs. For more information, see Using Cost
    /// Allocation Tags (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL)
    /// in the Amazon Web Services Billing and Cost Management User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<MonitoringScheduleTags>>,
}

/// The configuration object that specifies the monitoring schedule and defines
/// the monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfig {
    /// Defines the monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringJobDefinition")]
    pub monitoring_job_definition: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringJobDefinitionName")]
    pub monitoring_job_definition_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringType")]
    pub monitoring_type: Option<String>,
    /// Configuration details about the monitoring schedule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduleConfig")]
    pub schedule_config: Option<MonitoringScheduleMonitoringScheduleConfigScheduleConfig>,
}

/// Defines the monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinition {
    /// Configuration for monitoring constraints and monitoring statistics. These
    /// baseline resources are compared against the results of the current job from
    /// the series of jobs scheduled to collect data periodically.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baselineConfig")]
    pub baseline_config: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    /// Container image configuration object for the monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringAppSpecification")]
    pub monitoring_app_specification: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringAppSpecification>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringInputs")]
    pub monitoring_inputs: Option<Vec<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringInputs>>,
    /// The output configuration for monitoring jobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringOutputConfig")]
    pub monitoring_output_config: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfig>,
    /// Identifies the resources to deploy for a monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringResources")]
    pub monitoring_resources: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringResources>,
    /// Networking options for a job, such as network traffic encryption between
    /// containers, whether to allow inbound and outbound network calls to and from
    /// containers, and the VPC subnets and security groups to use for VPC-enabled
    /// jobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkConfig")]
    pub network_config: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionNetworkConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
    /// A time limit for how long the monitoring job is allowed to run before stopping.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stoppingCondition")]
    pub stopping_condition: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionStoppingCondition>,
}

/// Configuration for monitoring constraints and monitoring statistics. These
/// baseline resources are compared against the results of the current job from
/// the series of jobs scheduled to collect data periodically.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseliningJobName")]
    pub baselining_job_name: Option<String>,
    /// The constraints resource for a monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "constraintsResource")]
    pub constraints_resource: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfigConstraintsResource>,
    /// The statistics resource for a monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statisticsResource")]
    pub statistics_resource: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfigStatisticsResource>,
}

/// The constraints resource for a monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfigConstraintsResource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// The statistics resource for a monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionBaselineConfigStatisticsResource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// Container image configuration object for the monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringAppSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerArguments")]
    pub container_arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageURI")]
    pub image_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postAnalyticsProcessorSourceURI")]
    pub post_analytics_processor_source_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "recordPreprocessorSourceURI")]
    pub record_preprocessor_source_uri: Option<String>,
}

/// The inputs for a monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringInputs {
    /// Input object for the endpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointInput")]
    pub endpoint_input: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringInputsEndpointInput>,
}

/// Input object for the endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringInputsEndpointInput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endTimeOffset")]
    pub end_time_offset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointName")]
    pub endpoint_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeFeaturesAttribute")]
    pub exclude_features_attribute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featuresAttribute")]
    pub features_attribute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inferenceAttribute")]
    pub inference_attribute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "probabilityAttribute")]
    pub probability_attribute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "probabilityThresholdAttribute")]
    pub probability_threshold_attribute: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTimeOffset")]
    pub start_time_offset: Option<String>,
}

/// The output configuration for monitoring jobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringOutputs")]
    pub monitoring_outputs: Option<Vec<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfigMonitoringOutputs>>,
}

/// The output object for a monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfigMonitoringOutputs {
    /// Information about where and how you want to store the results of a monitoring
    /// job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Output")]
    pub s3_output: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfigMonitoringOutputsS3Output>,
}

/// Information about where and how you want to store the results of a monitoring
/// job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringOutputConfigMonitoringOutputsS3Output {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3UploadMode")]
    pub s3_upload_mode: Option<String>,
}

/// Identifies the resources to deploy for a monitoring job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringResources {
    /// Configuration for the cluster used to run model monitoring jobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterConfig")]
    pub cluster_config: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringResourcesClusterConfig>,
}

/// Configuration for the cluster used to run model monitoring jobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionMonitoringResourcesClusterConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceCount")]
    pub instance_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeKMSKeyID")]
    pub volume_kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSizeInGB")]
    pub volume_size_in_gb: Option<i64>,
}

/// Networking options for a job, such as network traffic encryption between
/// containers, whether to allow inbound and outbound network calls to and from
/// containers, and the VPC subnets and security groups to use for VPC-enabled
/// jobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionNetworkConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    /// Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs,
    /// hosted models, and compute resources have access to. You can control access
    /// to and from your resources by configuring a VPC. For more information, see
    /// Give SageMaker Access to Resources in your Amazon VPC (https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcConfig")]
    pub vpc_config: Option<MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionNetworkConfigVpcConfig>,
}

/// Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs,
/// hosted models, and compute resources have access to. You can control access
/// to and from your resources by configuring a VPC. For more information, see
/// Give SageMaker Access to Resources in your Amazon VPC (https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionNetworkConfigVpcConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

/// A time limit for how long the monitoring job is allowed to run before stopping.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigMonitoringJobDefinitionStoppingCondition {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRuntimeInSeconds")]
    pub max_runtime_in_seconds: Option<i64>,
}

/// Configuration details about the monitoring schedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleMonitoringScheduleConfigScheduleConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataAnalysisEndTime")]
    pub data_analysis_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataAnalysisStartTime")]
    pub data_analysis_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduleExpression")]
    pub schedule_expression: Option<String>,
}

/// A tag object that consists of a key and an optional value, used to manage
/// metadata for SageMaker Amazon Web Services resources.
/// 
/// You can add tags to notebook instances, training jobs, hyperparameter tuning
/// jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations,
/// and endpoints. For more information on adding tags to SageMaker resources,
/// see AddTags (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AddTags.html).
/// 
/// For more information on adding metadata to your Amazon Web Services resources
/// with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
/// For advice on best practices for managing Amazon Web Services resources with
/// tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services
/// Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// MonitoringScheduleStatus defines the observed state of MonitoringSchedule
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<MonitoringScheduleStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The time at which the monitoring job was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// A string, up to one KB in size, that contains the reason a monitoring job
    /// failed, if it failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// The time at which the monitoring job was last modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// Describes metadata on the last execution to run, if there was one.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastMonitoringExecutionSummary")]
    pub last_monitoring_execution_summary: Option<MonitoringScheduleStatusLastMonitoringExecutionSummary>,
    /// The status of an monitoring job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringScheduleStatus")]
    pub monitoring_schedule_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleStatusAckResourceMetadata {
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

/// Describes metadata on the last execution to run, if there was one.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MonitoringScheduleStatusLastMonitoringExecutionSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointName")]
    pub endpoint_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringExecutionStatus")]
    pub monitoring_execution_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringJobDefinitionName")]
    pub monitoring_job_definition_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringScheduleName")]
    pub monitoring_schedule_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "monitoringType")]
    pub monitoring_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processingJobARN")]
    pub processing_job_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scheduledTime")]
    pub scheduled_time: Option<String>,
}

