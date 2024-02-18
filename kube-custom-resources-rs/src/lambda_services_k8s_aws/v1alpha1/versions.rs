// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/lambda-controller/lambda.services.k8s.aws/v1alpha1/versions.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lambda.services.k8s.aws", version = "v1alpha1", kind = "Version", plural = "versions")]
#[kube(namespaced)]
#[kube(status = "VersionStatus")]
#[kube(schema = "disabled")]
pub struct VersionSpec {
    /// Only publish a version if the hash value matches the value that's specified.
    /// Use this option to avoid publishing a version if the function code has changed
    /// since you last updated it. You can get the hash for the version that you
    /// uploaded from the output of UpdateFunctionCode.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSHA256")]
    pub code_sha256: Option<String>,
    /// A description for the version to override the description in the function
    /// configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionEventInvokeConfig")]
    pub function_event_invoke_config: Option<VersionFunctionEventInvokeConfig>,
    /// The name of the Lambda function.
    /// 
    /// 
    /// Name formats
    /// 
    /// 
    ///    * Function name - MyFunction.
    /// 
    /// 
    ///    * Function ARN - arn:aws:lambda:us-west-2:123456789012:function:MyFunction.
    /// 
    /// 
    ///    * Partial ARN - 123456789012:function:MyFunction.
    /// 
    /// 
    /// The length constraint applies only to the full ARN. If you specify only the
    /// function name, it is limited to 64 characters in length.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionName")]
    pub function_name: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionRef")]
    pub function_ref: Option<VersionFunctionRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedConcurrencyConfig")]
    pub provisioned_concurrency_config: Option<VersionProvisionedConcurrencyConfig>,
    /// Only update the function if the revision ID matches the ID that's specified.
    /// Use this option to avoid publishing a version if the function configuration
    /// has changed since you last updated it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionID")]
    pub revision_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionEventInvokeConfig {
    /// A configuration object that specifies the destination of an event after Lambda
    /// processes it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationConfig")]
    pub destination_config: Option<VersionFunctionEventInvokeConfigDestinationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionName")]
    pub function_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumEventAgeInSeconds")]
    pub maximum_event_age_in_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumRetryAttempts")]
    pub maximum_retry_attempts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// A configuration object that specifies the destination of an event after Lambda
/// processes it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionEventInvokeConfigDestinationConfig {
    /// A destination for events that failed processing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onFailure")]
    pub on_failure: Option<VersionFunctionEventInvokeConfigDestinationConfigOnFailure>,
    /// A destination for events that were processed successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "onSuccess")]
    pub on_success: Option<VersionFunctionEventInvokeConfigDestinationConfigOnSuccess>,
}

/// A destination for events that failed processing.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionEventInvokeConfigDestinationConfigOnFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// A destination for events that were processed successfully.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionEventInvokeConfigDestinationConfigOnSuccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<VersionFunctionRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionFunctionRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionProvisionedConcurrencyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionName")]
    pub function_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "provisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// VersionStatus defines the observed state of Version
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<VersionStatusAckResourceMetadata>,
    /// The instruction set architecture that the function supports. Architecture
    /// is a string array with one of the valid values. The default architecture
    /// value is x86_64.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// The size of the function's deployment package, in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSize")]
    pub code_size: Option<i64>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<VersionStatusConditions>>,
    /// The function's dead letter queue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deadLetterConfig")]
    pub dead_letter_config: Option<VersionStatusDeadLetterConfig>,
    /// The function's environment variables (https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html).
    /// Omitted from CloudTrail logs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<VersionStatusEnvironment>,
    /// The size of the function’s /tmp directory in MB. The default value is 512,
    /// but it can be any whole number between 512 and 10,240 MB.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ephemeralStorage")]
    pub ephemeral_storage: Option<VersionStatusEphemeralStorage>,
    /// Connection settings for an Amazon EFS file system (https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemConfigs")]
    pub file_system_configs: Option<Vec<VersionStatusFileSystemConfigs>>,
    /// The function's Amazon Resource Name (ARN).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionARN")]
    pub function_arn: Option<String>,
    /// The function that Lambda calls to begin running your function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// The function's image configuration values.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfigResponse")]
    pub image_config_response: Option<VersionStatusImageConfigResponse>,
    /// The KMS key that's used to encrypt the function's environment variables.
    /// This key is returned only if you've configured a customer managed key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyARN")]
    pub kms_key_arn: Option<String>,
    /// The date and time that the function was last updated, in ISO-8601 format
    /// (https://www.w3.org/TR/NOTE-datetime) (YYYY-MM-DDThh:mm:ss.sTZD).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModified")]
    pub last_modified: Option<String>,
    /// The status of the last update that was performed on the function. This is
    /// first set to Successful after function creation completes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatus")]
    pub last_update_status: Option<String>,
    /// The reason for the last update that was performed on the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatusReason")]
    pub last_update_status_reason: Option<String>,
    /// The reason code for the last update that was performed on the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdateStatusReasonCode")]
    pub last_update_status_reason_code: Option<String>,
    /// The function's layers (https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<VersionStatusLayers>>,
    /// For Lambda@Edge functions, the ARN of the main function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterARN")]
    pub master_arn: Option<String>,
    /// The amount of memory available to the function at runtime.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySize")]
    pub memory_size: Option<i64>,
    /// The type of deployment package. Set to Image for container image and set
    /// Zip for .zip file archive.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "packageType")]
    pub package_type: Option<String>,
    /// The version of the Lambda function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    /// The function's execution role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The runtime environment for the Lambda function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// The ARN of the signing job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingJobARN")]
    pub signing_job_arn: Option<String>,
    /// The ARN of the signing profile version.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingProfileVersionARN")]
    pub signing_profile_version_arn: Option<String>,
    /// Set ApplyOn to PublishedVersions to create a snapshot of the initialized
    /// execution environment when you publish a function version. For more information,
    /// see Improving startup performance with Lambda SnapStart (https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapStart")]
    pub snap_start: Option<VersionStatusSnapStart>,
    /// The current state of the function. When the state is Inactive, you can reactivate
    /// the function by invoking it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The reason for the function's current state.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateReason")]
    pub state_reason: Option<String>,
    /// The reason code for the function's current state. When the code is Creating,
    /// you can't invoke or modify the function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stateReasonCode")]
    pub state_reason_code: Option<String>,
    /// The amount of time in seconds that Lambda allows a function to run before
    /// stopping it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// The function's X-Ray tracing configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfig")]
    pub tracing_config: Option<VersionStatusTracingConfig>,
    /// The version of the Lambda function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The function's networking configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcConfig")]
    pub vpc_config: Option<VersionStatusVpcConfig>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusAckResourceMetadata {
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

/// Condition is the common struct used by all CRDs managed by ACK service
/// controllers to indicate terminal states  of the CR and its backend AWS
/// service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type is the type of the Condition
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The function's dead letter queue.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusDeadLetterConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetARN")]
    pub target_arn: Option<String>,
}

/// The function's environment variables (https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html).
/// Omitted from CloudTrail logs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusEnvironment {
    /// Error messages for environment variables that couldn't be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<VersionStatusEnvironmentError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, String>>,
}

/// Error messages for environment variables that couldn't be applied.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusEnvironmentError {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// The size of the function’s /tmp directory in MB. The default value is 512,
/// but it can be any whole number between 512 and 10,240 MB.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusEphemeralStorage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// Details about the connection between a Lambda function and an Amazon EFS
/// file system (https://docs.aws.amazon.com/lambda/latest/dg/configuration-filesystem.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusFileSystemConfigs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "localMountPath")]
    pub local_mount_path: Option<String>,
}

/// The function's image configuration values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusImageConfigResponse {
    /// Error response to GetFunctionConfiguration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<VersionStatusImageConfigResponseError>,
    /// Configuration values that override the container image Dockerfile settings.
    /// For more information, see Container image settings (https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-parms).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfig")]
    pub image_config: Option<VersionStatusImageConfigResponseImageConfig>,
}

/// Error response to GetFunctionConfiguration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusImageConfigResponseError {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Configuration values that override the container image Dockerfile settings.
/// For more information, see Container image settings (https://docs.aws.amazon.com/lambda/latest/dg/images-create.html#images-parms).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusImageConfigResponseImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entryPoint")]
    pub entry_point: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workingDirectory")]
    pub working_directory: Option<String>,
}

/// An Lambda layer (https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusLayers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeSize")]
    pub code_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingJobARN")]
    pub signing_job_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signingProfileVersionARN")]
    pub signing_profile_version_arn: Option<String>,
}

/// Set ApplyOn to PublishedVersions to create a snapshot of the initialized
/// execution environment when you publish a function version. For more information,
/// see Improving startup performance with Lambda SnapStart (https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusSnapStart {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyOn")]
    pub apply_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optimizationStatus")]
    pub optimization_status: Option<String>,
}

/// The function's X-Ray tracing configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusTracingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// The function's networking configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VersionStatusVpcConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

