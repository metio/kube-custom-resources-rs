// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/models.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ModelSpec defines the desired state of Model.
/// 
/// 
/// The properties of a model as returned by the Search (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_Search.html)
/// API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "Model", plural = "models")]
#[kube(namespaced)]
#[kube(status = "ModelStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ModelSpec {
    /// Specifies the containers in the inference pipeline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ModelContainers>>,
    /// Isolates the model container. No inbound or outbound network calls can be
    /// made to or from the model container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableNetworkIsolation")]
    pub enable_network_isolation: Option<bool>,
    /// The Amazon Resource Name (ARN) of the IAM role that SageMaker can assume
    /// to access model artifacts and docker image for deployment on ML compute instances
    /// or for batch transform jobs. Deploying on ML compute instances is part of
    /// model hosting. For more information, see SageMaker Roles (https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html).
    /// 
    /// 
    /// To be able to pass this role to SageMaker, the caller of this API must have
    /// the iam:PassRole permission.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionRoleARN")]
    pub execution_role_arn: Option<String>,
    /// Specifies details of how containers in a multi-container endpoint are called.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inferenceExecutionConfig")]
    pub inference_execution_config: Option<ModelInferenceExecutionConfig>,
    /// The name of the new model.
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// The location of the primary docker image containing inference code, associated
    /// artifacts, and custom environment map that the inference code uses when the
    /// model is deployed for predictions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryContainer")]
    pub primary_container: Option<ModelPrimaryContainer>,
    /// An array of key-value pairs. You can use tags to categorize your Amazon Web
    /// Services resources in different ways, for example, by purpose, owner, or
    /// environment. For more information, see Tagging Amazon Web Services Resources
    /// (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ModelTags>>,
    /// A VpcConfig (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html)
    /// object that specifies the VPC that you want your model to connect to. Control
    /// access to and from your model container by configuring the VPC. VpcConfig
    /// is used in hosting services and in batch transform. For more information,
    /// see Protect Endpoints by Using an Amazon Virtual Private Cloud (https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html)
    /// and Protect Data in Batch Transform Jobs by Using an Amazon Virtual Private
    /// Cloud (https://docs.aws.amazon.com/sagemaker/latest/dg/batch-vpc.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcConfig")]
    pub vpc_config: Option<ModelVpcConfig>,
}

/// Describes the container, as part of model definition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerHostname")]
    pub container_hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specifies whether the model container is in Amazon ECR or a private Docker
    /// registry accessible from your Amazon Virtual Private Cloud (VPC).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfig")]
    pub image_config: Option<ModelContainersImageConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inferenceSpecificationName")]
    pub inference_specification_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Specifies the location of ML model data to deploy. If specified, you must
    /// specify one and only one of the available data sources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelDataSource")]
    pub model_data_source: Option<ModelContainersModelDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelDataURL")]
    pub model_data_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelPackageName")]
    pub model_package_name: Option<String>,
    /// Specifies additional configuration for hosting multi-model endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiModelConfig")]
    pub multi_model_config: Option<ModelContainersMultiModelConfig>,
}

/// Specifies whether the model container is in Amazon ECR or a private Docker
/// registry accessible from your Amazon Virtual Private Cloud (VPC).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryAccessMode")]
    pub repository_access_mode: Option<String>,
    /// Specifies an authentication configuration for the private docker registry
    /// where your model image is hosted. Specify a value for this property only
    /// if you specified Vpc as the value for the RepositoryAccessMode field of the
    /// ImageConfig object that you passed to a call to CreateModel and the private
    /// Docker registry where the model image is hosted requires authentication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryAuthConfig")]
    pub repository_auth_config: Option<ModelContainersImageConfigRepositoryAuthConfig>,
}

/// Specifies an authentication configuration for the private docker registry
/// where your model image is hosted. Specify a value for this property only
/// if you specified Vpc as the value for the RepositoryAccessMode field of the
/// ImageConfig object that you passed to a call to CreateModel and the private
/// Docker registry where the model image is hosted requires authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersImageConfigRepositoryAuthConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryCredentialsProviderARN")]
    pub repository_credentials_provider_arn: Option<String>,
}

/// Specifies the location of ML model data to deploy. If specified, you must
/// specify one and only one of the available data sources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersModelDataSource {
    /// Specifies the S3 location of ML model data to deploy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataSource")]
    pub s3_data_source: Option<ModelContainersModelDataSourceS3DataSource>,
}

/// Specifies the S3 location of ML model data to deploy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersModelDataSourceS3DataSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionType")]
    pub compression_type: Option<String>,
    /// The access configuration file to control access to the ML model. You can
    /// explicitly accept the model end-user license agreement (EULA) within the
    /// ModelAccessConfig.
    /// 
    /// 
    ///    * If you are a Jumpstart user, see the End-user license agreements (https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-choose.html#jumpstart-foundation-models-choose-eula)
    ///    section for more details on accepting the EULA.
    /// 
    /// 
    ///    * If you are an AutoML user, see the Optional Parameters section of Create
    ///    an AutoML job to fine-tune text generation models using the API for details
    ///    on How to set the EULA acceptance when fine-tuning a model using the AutoML
    ///    API (https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-create-experiment-finetune-llms.html#autopilot-llms-finetuning-api-optional-params).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelAccessConfig")]
    pub model_access_config: Option<ModelContainersModelDataSourceS3DataSourceModelAccessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataType")]
    pub s3_data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// The access configuration file to control access to the ML model. You can
/// explicitly accept the model end-user license agreement (EULA) within the
/// ModelAccessConfig.
/// 
/// 
///    * If you are a Jumpstart user, see the End-user license agreements (https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-choose.html#jumpstart-foundation-models-choose-eula)
///    section for more details on accepting the EULA.
/// 
/// 
///    * If you are an AutoML user, see the Optional Parameters section of Create
///    an AutoML job to fine-tune text generation models using the API for details
///    on How to set the EULA acceptance when fine-tuning a model using the AutoML
///    API (https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-create-experiment-finetune-llms.html#autopilot-llms-finetuning-api-optional-params).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersModelDataSourceS3DataSourceModelAccessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acceptEula")]
    pub accept_eula: Option<bool>,
}

/// Specifies additional configuration for hosting multi-model endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelContainersMultiModelConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelCacheSetting")]
    pub model_cache_setting: Option<String>,
}

/// Specifies details of how containers in a multi-container endpoint are called.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelInferenceExecutionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// The location of the primary docker image containing inference code, associated
/// artifacts, and custom environment map that the inference code uses when the
/// model is deployed for predictions.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerHostname")]
    pub container_hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Specifies whether the model container is in Amazon ECR or a private Docker
    /// registry accessible from your Amazon Virtual Private Cloud (VPC).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageConfig")]
    pub image_config: Option<ModelPrimaryContainerImageConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inferenceSpecificationName")]
    pub inference_specification_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Specifies the location of ML model data to deploy. If specified, you must
    /// specify one and only one of the available data sources.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelDataSource")]
    pub model_data_source: Option<ModelPrimaryContainerModelDataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelDataURL")]
    pub model_data_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelPackageName")]
    pub model_package_name: Option<String>,
    /// Specifies additional configuration for hosting multi-model endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiModelConfig")]
    pub multi_model_config: Option<ModelPrimaryContainerMultiModelConfig>,
}

/// Specifies whether the model container is in Amazon ECR or a private Docker
/// registry accessible from your Amazon Virtual Private Cloud (VPC).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerImageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryAccessMode")]
    pub repository_access_mode: Option<String>,
    /// Specifies an authentication configuration for the private docker registry
    /// where your model image is hosted. Specify a value for this property only
    /// if you specified Vpc as the value for the RepositoryAccessMode field of the
    /// ImageConfig object that you passed to a call to CreateModel and the private
    /// Docker registry where the model image is hosted requires authentication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryAuthConfig")]
    pub repository_auth_config: Option<ModelPrimaryContainerImageConfigRepositoryAuthConfig>,
}

/// Specifies an authentication configuration for the private docker registry
/// where your model image is hosted. Specify a value for this property only
/// if you specified Vpc as the value for the RepositoryAccessMode field of the
/// ImageConfig object that you passed to a call to CreateModel and the private
/// Docker registry where the model image is hosted requires authentication.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerImageConfigRepositoryAuthConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoryCredentialsProviderARN")]
    pub repository_credentials_provider_arn: Option<String>,
}

/// Specifies the location of ML model data to deploy. If specified, you must
/// specify one and only one of the available data sources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerModelDataSource {
    /// Specifies the S3 location of ML model data to deploy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataSource")]
    pub s3_data_source: Option<ModelPrimaryContainerModelDataSourceS3DataSource>,
}

/// Specifies the S3 location of ML model data to deploy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerModelDataSourceS3DataSource {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "compressionType")]
    pub compression_type: Option<String>,
    /// The access configuration file to control access to the ML model. You can
    /// explicitly accept the model end-user license agreement (EULA) within the
    /// ModelAccessConfig.
    /// 
    /// 
    ///    * If you are a Jumpstart user, see the End-user license agreements (https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-choose.html#jumpstart-foundation-models-choose-eula)
    ///    section for more details on accepting the EULA.
    /// 
    /// 
    ///    * If you are an AutoML user, see the Optional Parameters section of Create
    ///    an AutoML job to fine-tune text generation models using the API for details
    ///    on How to set the EULA acceptance when fine-tuning a model using the AutoML
    ///    API (https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-create-experiment-finetune-llms.html#autopilot-llms-finetuning-api-optional-params).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelAccessConfig")]
    pub model_access_config: Option<ModelPrimaryContainerModelDataSourceS3DataSourceModelAccessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DataType")]
    pub s3_data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3URI")]
    pub s3_uri: Option<String>,
}

/// The access configuration file to control access to the ML model. You can
/// explicitly accept the model end-user license agreement (EULA) within the
/// ModelAccessConfig.
/// 
/// 
///    * If you are a Jumpstart user, see the End-user license agreements (https://docs.aws.amazon.com/sagemaker/latest/dg/jumpstart-foundation-models-choose.html#jumpstart-foundation-models-choose-eula)
///    section for more details on accepting the EULA.
/// 
/// 
///    * If you are an AutoML user, see the Optional Parameters section of Create
///    an AutoML job to fine-tune text generation models using the API for details
///    on How to set the EULA acceptance when fine-tuning a model using the AutoML
///    API (https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-create-experiment-finetune-llms.html#autopilot-llms-finetuning-api-optional-params).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerModelDataSourceS3DataSourceModelAccessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acceptEula")]
    pub accept_eula: Option<bool>,
}

/// Specifies additional configuration for hosting multi-model endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelPrimaryContainerMultiModelConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelCacheSetting")]
    pub model_cache_setting: Option<String>,
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
pub struct ModelTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A VpcConfig (https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_VpcConfig.html)
/// object that specifies the VPC that you want your model to connect to. Control
/// access to and from your model container by configuring the VPC. VpcConfig
/// is used in hosting services and in batch transform. For more information,
/// see Protect Endpoints by Using an Amazon Virtual Private Cloud (https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html)
/// and Protect Data in Batch Transform Jobs by Using an Amazon Virtual Private
/// Cloud (https://docs.aws.amazon.com/sagemaker/latest/dg/batch-vpc.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelVpcConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

/// ModelStatus defines the observed state of Model
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ModelStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ModelStatusAckResourceMetadata {
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

