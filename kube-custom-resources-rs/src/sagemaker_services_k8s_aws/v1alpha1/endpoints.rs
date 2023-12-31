// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/endpoints.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// EndpointSpec defines the desired state of Endpoint. 
///  A hosted endpoint for real-time inference.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "Endpoint", plural = "endpoints")]
#[kube(namespaced)]
#[kube(status = "EndpointStatus")]
#[kube(schema = "disabled")]
pub struct EndpointSpec {
    /// The deployment configuration for an endpoint, which contains the desired deployment strategy and rollback configurations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentConfig")]
    pub deployment_config: Option<EndpointDeploymentConfig>,
    /// The name of an endpoint configuration. For more information, see CreateEndpointConfig.
    #[serde(rename = "endpointConfigName")]
    pub endpoint_config_name: String,
    /// The name of the endpoint.The name must be unique within an Amazon Web Services Region in your Amazon Web Services account. The name is case-insensitive in CreateEndpoint, but the case is preserved and must be matched in .
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
    /// An array of key-value pairs. You can use tags to categorize your Amazon Web Services resources in different ways, for example, by purpose, owner, or environment. For more information, see Tagging Amazon Web Services Resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<EndpointTags>>,
}

/// The deployment configuration for an endpoint, which contains the desired deployment strategy and rollback configurations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfig {
    /// Automatic rollback configuration for handling endpoint deployment failures and recovery.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoRollbackConfiguration")]
    pub auto_rollback_configuration: Option<EndpointDeploymentConfigAutoRollbackConfiguration>,
    /// Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blueGreenUpdatePolicy")]
    pub blue_green_update_policy: Option<EndpointDeploymentConfigBlueGreenUpdatePolicy>,
}

/// Automatic rollback configuration for handling endpoint deployment failures and recovery.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigAutoRollbackConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<EndpointDeploymentConfigAutoRollbackConfigurationAlarms>>,
}

/// An Amazon CloudWatch alarm configured to monitor metrics on an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigAutoRollbackConfigurationAlarms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "alarmName")]
    pub alarm_name: Option<String>,
}

/// Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumExecutionTimeoutInSeconds")]
    pub maximum_execution_timeout_in_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "terminationWaitInSeconds")]
    pub termination_wait_in_seconds: Option<i64>,
    /// Defines the traffic routing strategy during an endpoint deployment to shift traffic from the old fleet to the new fleet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "trafficRoutingConfiguration")]
    pub traffic_routing_configuration: Option<EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration>,
}

/// Defines the traffic routing strategy during an endpoint deployment to shift traffic from the old fleet to the new fleet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration {
    /// Specifies the endpoint capacity to activate for production.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "canarySize")]
    pub canary_size: Option<EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationCanarySize>,
    /// Specifies the endpoint capacity to activate for production.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linearStepSize")]
    pub linear_step_size: Option<EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationLinearStepSize>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitIntervalInSeconds")]
    pub wait_interval_in_seconds: Option<i64>,
}

/// Specifies the endpoint capacity to activate for production.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationCanarySize {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// Specifies the endpoint capacity to activate for production.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfigurationLinearStepSize {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type_")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// A tag object that consists of a key and an optional value, used to manage metadata for SageMaker Amazon Web Services resources. 
///  You can add tags to notebook instances, training jobs, hyperparameter tuning jobs, batch transform jobs, models, labeling jobs, work teams, endpoint configurations, and endpoints. For more information on adding tags to SageMaker resources, see AddTags. 
///  For more information on adding metadata to your Amazon Web Services resources with tagging, see Tagging Amazon Web Services resources (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html). For advice on best practices for managing Amazon Web Services resources with tagging, see Tagging Best Practices: Implement an Effective Amazon Web Services Resource Tagging Strategy (https://d1.awsstatic.com/whitepapers/aws-tagging-best-practices.pdf).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// EndpointStatus defines the observed state of Endpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<EndpointStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that contains a collection of `ackv1alpha1.Condition` objects that describe the various terminal states of the CR and its backend AWS service API resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<EndpointStatusConditions>>,
    /// A timestamp that shows when the endpoint was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The status of the endpoint. 
    ///  * OutOfService: Endpoint is not available to take incoming requests. 
    ///  * Creating: CreateEndpoint is executing. 
    ///  * Updating: UpdateEndpoint or UpdateEndpointWeightsAndCapacities is executing. 
    ///  * SystemUpdating: Endpoint is undergoing maintenance and cannot be updated or deleted or re-scaled until it has completed. This maintenance operation does not change any customer-specified values such as VPC config, KMS encryption, model, instance type, or instance count. 
    ///  * RollingBack: Endpoint fails to scale up or down or change its variant weight and is in the process of rolling back to its previous configuration. Once the rollback completes, endpoint returns to an InService status. This transitional status only applies to an endpoint that has autoscaling enabled and is undergoing variant weight or capacity changes as part of an UpdateEndpointWeightsAndCapacities call or when the UpdateEndpointWeightsAndCapacities operation is called explicitly. 
    ///  * InService: Endpoint is available to process incoming requests. 
    ///  * Deleting: DeleteEndpoint is executing. 
    ///  * Failed: Endpoint could not be created, updated, or re-scaled. Use DescribeEndpointOutput$FailureReason for information about the failure. DeleteEndpoint is the only operation that can be performed on a failed endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointStatus")]
    pub endpoint_status: Option<String>,
    /// If the status of the endpoint is Failed, the reason why it failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// A timestamp that shows when the endpoint was last modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    /// Returns the summary of an in-progress deployment. This field is only returned when the endpoint is creating or updating with a new endpoint configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingDeploymentSummary")]
    pub pending_deployment_summary: Option<EndpointStatusPendingDeploymentSummary>,
    /// An array of ProductionVariantSummary objects, one for each model hosted behind this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productionVariants")]
    pub production_variants: Option<Vec<EndpointStatusProductionVariants>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member that is used to contain resource sync state, account ownership, constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a globally-unique identifier and is set only by the ACK service controller once the controller has orchestrated the creation of the resource OR when it has verified that an "adopted" resource (a resource where the ARN annotation was set by the Kubernetes user on the CR) exists and matches the supplied CR's Spec field values. TODO(vijat@): Find a better strategy for resources that do not have ARN in CreateOutputResponse https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

/// Condition is the common struct used by all CRDs managed by ACK service controllers to indicate terminal states  of the CR and its backend AWS service API resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusConditions {
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

/// Returns the summary of an in-progress deployment. This field is only returned when the endpoint is creating or updating with a new endpoint configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummary {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointConfigName")]
    pub endpoint_config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productionVariants")]
    pub production_variants: Option<Vec<EndpointStatusPendingDeploymentSummaryProductionVariants>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// The production variant summary for a deployment when an endpoint is creating or updating with the CreateEndpoint or UpdateEndpoint operations. Describes the VariantStatus , weight and capacity for a production variant associated with an endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummaryProductionVariants {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acceleratorType")]
    pub accelerator_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentInstanceCount")]
    pub current_instance_count: Option<i64>,
    /// Specifies the serverless configuration for an endpoint variant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentServerlessConfig")]
    pub current_serverless_config: Option<EndpointStatusPendingDeploymentSummaryProductionVariantsCurrentServerlessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentWeight")]
    pub current_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deployedImages")]
    pub deployed_images: Option<Vec<EndpointStatusPendingDeploymentSummaryProductionVariantsDeployedImages>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredInstanceCount")]
    pub desired_instance_count: Option<i64>,
    /// Specifies the serverless configuration for an endpoint variant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredServerlessConfig")]
    pub desired_serverless_config: Option<EndpointStatusPendingDeploymentSummaryProductionVariantsDesiredServerlessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredWeight")]
    pub desired_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "variantName")]
    pub variant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "variantStatus")]
    pub variant_status: Option<Vec<EndpointStatusPendingDeploymentSummaryProductionVariantsVariantStatus>>,
}

/// Specifies the serverless configuration for an endpoint variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummaryProductionVariantsCurrentServerlessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrency")]
    pub max_concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySizeInMB")]
    pub memory_size_in_mb: Option<i64>,
}

/// Gets the Amazon EC2 Container Registry path of the docker image of the model that is hosted in this ProductionVariant. 
///  If you used the registry/repository[:tag] form to specify the image path of the primary container when you created the model hosted in this ProductionVariant, the path resolves to a path of the form registry/repository[@digest]. A digest is a hash value that identifies a specific version of an image. For information about Amazon ECR paths, see Pulling an Image (https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-pull-ecr-image.html) in the Amazon ECR User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummaryProductionVariantsDeployedImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolutionTime")]
    pub resolution_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedImage")]
    pub resolved_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specifiedImage")]
    pub specified_image: Option<String>,
}

/// Specifies the serverless configuration for an endpoint variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummaryProductionVariantsDesiredServerlessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrency")]
    pub max_concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySizeInMB")]
    pub memory_size_in_mb: Option<i64>,
}

/// Describes the status of the production variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusPendingDeploymentSummaryProductionVariantsVariantStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

/// Describes weight and capacities for a production variant associated with an endpoint. If you sent a request to the UpdateEndpointWeightsAndCapacities API and the endpoint status is Updating, you get different desired and current values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusProductionVariants {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentInstanceCount")]
    pub current_instance_count: Option<i64>,
    /// Specifies the serverless configuration for an endpoint variant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentServerlessConfig")]
    pub current_serverless_config: Option<EndpointStatusProductionVariantsCurrentServerlessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentWeight")]
    pub current_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deployedImages")]
    pub deployed_images: Option<Vec<EndpointStatusProductionVariantsDeployedImages>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredInstanceCount")]
    pub desired_instance_count: Option<i64>,
    /// Specifies the serverless configuration for an endpoint variant.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredServerlessConfig")]
    pub desired_serverless_config: Option<EndpointStatusProductionVariantsDesiredServerlessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredWeight")]
    pub desired_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "variantName")]
    pub variant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "variantStatus")]
    pub variant_status: Option<Vec<EndpointStatusProductionVariantsVariantStatus>>,
}

/// Specifies the serverless configuration for an endpoint variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusProductionVariantsCurrentServerlessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrency")]
    pub max_concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySizeInMB")]
    pub memory_size_in_mb: Option<i64>,
}

/// Gets the Amazon EC2 Container Registry path of the docker image of the model that is hosted in this ProductionVariant. 
///  If you used the registry/repository[:tag] form to specify the image path of the primary container when you created the model hosted in this ProductionVariant, the path resolves to a path of the form registry/repository[@digest]. A digest is a hash value that identifies a specific version of an image. For information about Amazon ECR paths, see Pulling an Image (https://docs.aws.amazon.com/AmazonECR/latest/userguide/docker-pull-ecr-image.html) in the Amazon ECR User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusProductionVariantsDeployedImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolutionTime")]
    pub resolution_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedImage")]
    pub resolved_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specifiedImage")]
    pub specified_image: Option<String>,
}

/// Specifies the serverless configuration for an endpoint variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusProductionVariantsDesiredServerlessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrency")]
    pub max_concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memorySizeInMB")]
    pub memory_size_in_mb: Option<i64>,
}

/// Describes the status of the production variant.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EndpointStatusProductionVariantsVariantStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusMessage")]
    pub status_message: Option<String>,
}

