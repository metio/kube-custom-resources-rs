// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/processingjobs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ProcessingJobSpec defines the desired state of ProcessingJob.
/// 
/// 
/// An Amazon SageMaker processing job that is used to analyze data and evaluate
/// models. For more information, see Process Data and Evaluate Models (https://docs.aws.amazon.com/sagemaker/latest/dg/processing-job.html).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "ProcessingJob", plural = "processingjobs")]
#[kube(namespaced)]
#[kube(status = "ProcessingJobStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ProcessingJobSpec {
    /// Configures the processing job to run a specified Docker container image.
    #[serde(rename = "appSpecification")]
    pub app_specification: ProcessingJobAppSpecification,
    /// The environment variables to set in the Docker container. Up to 100 key and
    /// values entries in the map are supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    /// Associates a SageMaker job as a trial component with an experiment and trial.
    /// Specified when you call the following APIs:
    /// 
    /// 
    ///    * CreateProcessingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateProcessingJob.html)
    /// 
    /// 
    ///    * CreateTrainingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTrainingJob.html)
    /// 
    /// 
    ///    * CreateTransformJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTransformJob.html)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentConfig")]
    pub experiment_config: Option<ProcessingJobExperimentConfig>,
    /// Networking options for a processing job, such as whether to allow inbound
    /// and outbound network calls to and from processing containers, and the VPC
    /// subnets and security groups to use for VPC-enabled processing jobs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkConfig")]
    pub network_config: Option<ProcessingJobNetworkConfig>,
    /// An array of inputs configuring the data to download into the processing container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processingInputs")]
    pub processing_inputs: Option<Vec<ProcessingJobProcessingInputs>>,
    /// The name of the processing job. The name must be unique within an Amazon
    /// Web Services Region in the Amazon Web Services account.
    #[serde(rename = "processingJobName")]
    pub processing_job_name: String,
    /// Output configuration for the processing job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processingOutputConfig")]
    pub processing_output_config: Option<ProcessingJobProcessingOutputConfig>,
    /// Identifies the resources, ML compute instances, and ML storage volumes to
    /// deploy for a processing job. In distributed training, you specify more than
    /// one instance.
    #[serde(rename = "processingResources")]
    pub processing_resources: ProcessingJobProcessingResources,
    /// The Amazon Resource Name (ARN) of an IAM role that Amazon SageMaker can assume
    /// to perform tasks on your behalf.
    #[serde(rename = "roleARN")]
    pub role_arn: String,
    /// The time limit for how long the processing job is allowed to run.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stoppingCondition")]
    pub stopping_condition: Option<ProcessingJobStoppingCondition>,
    /// (Optional) An array of key-value pairs. For more information, see Using Cost
    /// Allocation Tags (https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html#allocation-whatURL)
    /// in the Amazon Web Services Billing and Cost Management User Guide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ProcessingJobTags>>,
}

/// Configures the processing job to run a specified Docker container image.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobAppSpecification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerArguments")]
    pub container_arguments: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerEntrypoint")]
    pub container_entrypoint: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageURI")]
    pub image_uri: Option<String>,
}

/// Associates a SageMaker job as a trial component with an experiment and trial.
/// Specified when you call the following APIs:
/// 
/// 
///    * CreateProcessingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateProcessingJob.html)
/// 
/// 
///    * CreateTrainingJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTrainingJob.html)
/// 
/// 
///    * CreateTransformJob (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_CreateTransformJob.html)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobExperimentConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentName")]
    pub experiment_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trialComponentDisplayName")]
    pub trial_component_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trialName")]
    pub trial_name: Option<String>,
}

/// Networking options for a processing job, such as whether to allow inbound
/// and outbound network calls to and from processing containers, and the VPC
/// subnets and security groups to use for VPC-enabled processing jobs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobNetworkConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableInterContainerTrafficEncryption")]
    pub enable_inter_container_traffic_encryption: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    /// Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs,
    /// hosted models, and compute resources have access to. You can control access
    /// to and from your resources by configuring a VPC. For more information, see
    /// Give SageMaker Access to Resources in your Amazon VPC (https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcConfig")]
    pub vpc_config: Option<ProcessingJobNetworkConfigVpcConfig>,
}

/// Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs,
/// hosted models, and compute resources have access to. You can control access
/// to and from your resources by configuring a VPC. For more information, see
/// Give SageMaker Access to Resources in your Amazon VPC (https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobNetworkConfigVpcConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

/// The inputs for a processing job. The processing input must specify exactly
/// one of either S3Input or DatasetDefinition types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingInputs {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appManaged")]
    pub app_managed: Option<bool>,
    /// Configuration for Dataset Definition inputs. The Dataset Definition input
    /// must specify exactly one of either AthenaDatasetDefinition or RedshiftDatasetDefinition
    /// types.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "datasetDefinition")]
    pub dataset_definition: Option<ProcessingJobProcessingInputsDatasetDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inputName")]
    pub input_name: Option<String>,
    /// Configuration for downloading input data from Amazon S3 into the processing
    /// container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Input")]
    pub s3_input: Option<ProcessingJobProcessingInputsS3Input>,
}

/// Configuration for Dataset Definition inputs. The Dataset Definition input
/// must specify exactly one of either AthenaDatasetDefinition or RedshiftDatasetDefinition
/// types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingInputsDatasetDefinition {
    /// Configuration for Athena Dataset Definition input.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "athenaDatasetDefinition")]
    pub athena_dataset_definition: Option<ProcessingJobProcessingInputsDatasetDefinitionAthenaDatasetDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataDistributionType")]
    pub data_distribution_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inputMode")]
    pub input_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    /// Configuration for Redshift Dataset Definition input.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "redshiftDatasetDefinition")]
    pub redshift_dataset_definition: Option<ProcessingJobProcessingInputsDatasetDefinitionRedshiftDatasetDefinition>,
}

/// Configuration for Athena Dataset Definition input.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingInputsDatasetDefinitionAthenaDatasetDefinition {
    /// The name of the data catalog used in Athena query execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    /// The name of the database used in the Athena query execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The compression used for Athena query results.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputCompression")]
    pub output_compression: Option<String>,
    /// The data storage format for Athena query results.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputFormat")]
    pub output_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputS3URI")]
    pub output_s3uri: Option<String>,
    /// The SQL query statements, to be executed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryString")]
    pub query_string: Option<String>,
    /// The name of the workgroup in which the Athena query is being started.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workGroup")]
    pub work_group: Option<String>,
}

/// Configuration for Redshift Dataset Definition input.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingInputsDatasetDefinitionRedshiftDatasetDefinition {
    /// The Redshift cluster Identifier.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterID")]
    pub cluster_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterRoleARN")]
    pub cluster_role_arn: Option<String>,
    /// The name of the Redshift database used in Redshift query execution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// The database user name used in Redshift query execution.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dbUser")]
    pub db_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The compression used for Redshift query results.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputCompression")]
    pub output_compression: Option<String>,
    /// The data storage format for Redshift query results.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputFormat")]
    pub output_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputS3URI")]
    pub output_s3uri: Option<String>,
    /// The SQL query statements to be executed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryString")]
    pub query_string: Option<String>,
}

/// Configuration for downloading input data from Amazon S3 into the processing
/// container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingInputsS3Input {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3CompressionType")]
    pub s3_compression_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataDistributionType")]
    pub s3_data_distribution_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataType")]
    pub s3_data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3InputMode")]
    pub s3_input_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// Output configuration for the processing job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingOutputConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<ProcessingJobProcessingOutputConfigOutputs>>,
}

/// Describes the results of a processing job. The processing output must specify
/// exactly one of either S3Output or FeatureStoreOutput types.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingOutputConfigOutputs {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appManaged")]
    pub app_managed: Option<bool>,
    /// Configuration for processing job outputs in Amazon SageMaker Feature Store.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureStoreOutput")]
    pub feature_store_output: Option<ProcessingJobProcessingOutputConfigOutputsFeatureStoreOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outputName")]
    pub output_name: Option<String>,
    /// Configuration for uploading output data to Amazon S3 from the processing
    /// container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3Output")]
    pub s3_output: Option<ProcessingJobProcessingOutputConfigOutputsS3Output>,
}

/// Configuration for processing job outputs in Amazon SageMaker Feature Store.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingOutputConfigOutputsFeatureStoreOutput {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "featureGroupName")]
    pub feature_group_name: Option<String>,
}

/// Configuration for uploading output data to Amazon S3 from the processing
/// container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingOutputConfigOutputsS3Output {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localPath")]
    pub local_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3UploadMode")]
    pub s3_upload_mode: Option<String>,
}

/// Identifies the resources, ML compute instances, and ML storage volumes to
/// deploy for a processing job. In distributed training, you specify more than
/// one instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingResources {
    /// Configuration for the cluster used to run a processing job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterConfig")]
    pub cluster_config: Option<ProcessingJobProcessingResourcesClusterConfig>,
}

/// Configuration for the cluster used to run a processing job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobProcessingResourcesClusterConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceCount")]
    pub instance_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeKMSKeyID")]
    pub volume_kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSizeInGB")]
    pub volume_size_in_gb: Option<i64>,
}

/// The time limit for how long the processing job is allowed to run.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobStoppingCondition {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRuntimeInSeconds")]
    pub max_runtime_in_seconds: Option<i64>,
}

/// A tag object that consists of a key and an optional value, used to manage
/// metadata for SageMaker Amazon Web Services resources.
/// 
/// 
/// You can add tags to notebook instances, training jobs, hyperparameter tuning
/// jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations,
/// and endpoints. For more information on adding tags to SageMaker resources,
/// see AddTags (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_AddTags.html).
/// 
/// 
/// For more information on adding metadata to your Amazon Web Services resources
/// with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
/// For advice on best practices for managing Amazon Web Services resources with
/// tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services
/// Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// ProcessingJobStatus defines the observed state of ProcessingJob
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ProcessingJobStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A string, up to one KB in size, that contains the reason a processing job
    /// failed, if it failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Provides the status of a processing job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "processingJobStatus")]
    pub processing_job_status: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProcessingJobStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse
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

