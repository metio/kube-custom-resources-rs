// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/domains.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DomainSpec defines the desired state of Domain.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "Domain", plural = "domains")]
#[kube(namespaced)]
#[kube(status = "DomainStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DomainSpec {
    /// Specifies the VPC used for non-EFS traffic. The default value is PublicInternetOnly.
    /// 
    ///    * PublicInternetOnly - Non-EFS traffic is through a VPC managed by Amazon
    ///    SageMaker, which allows direct internet access
    /// 
    ///    * VpcOnly - All traffic is through the specified VPC and subnets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appNetworkAccessType")]
    pub app_network_access_type: Option<String>,
    /// The entity that creates and manages the required security groups for inter-app
    /// communication in VPCOnly mode. Required when CreateDomain.AppNetworkAccessType
    /// is VPCOnly and DomainSettings.RStudioServerProDomainSettings.DomainExecutionRoleArn
    /// is provided. If setting up the domain for use with RStudio, this value must
    /// be set to Service.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appSecurityGroupManagement")]
    pub app_security_group_management: Option<String>,
    /// The mode of authentication that members use to access the domain.
    #[serde(rename = "authMode")]
    pub auth_mode: String,
    /// The default settings to use to create a user profile when UserSettings isn't
    /// specified in the call to the CreateUserProfile API.
    /// 
    /// SecurityGroups is aggregated when specified in both calls. For all other
    /// settings in UserSettings, the values specified in CreateUserProfile take
    /// precedence over those specified in CreateDomain.
    #[serde(rename = "defaultUserSettings")]
    pub default_user_settings: DomainDefaultUserSettings,
    /// A name for the domain.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9](-*[a-zA-Z0-9]){0,62}$`
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// A collection of Domain settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainSettings")]
    pub domain_settings: Option<DomainDomainSettings>,
    /// Use KmsKeyId.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9:/_-]*$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "homeEFSFileSystemKMSKeyID")]
    pub home_efs_file_system_kms_key_id: Option<String>,
    /// SageMaker uses Amazon Web Services KMS to encrypt EFS and EBS volumes attached
    /// to the domain with an Amazon Web Services managed key by default. For more
    /// control, specify a customer managed key.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9:/_-]*$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The VPC subnets that the domain uses for communication.
    #[serde(rename = "subnetIDs")]
    pub subnet_i_ds: Vec<String>,
    /// Tags to associated with the Domain. Each tag consists of a key and an optional
    /// value. Tag keys must be unique per resource. Tags are searchable using the
    /// Search API.
    /// 
    /// Tags that you specify for the Domain are also added to all Apps that the
    /// Domain launches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DomainTags>>,
    /// The ID of the Amazon Virtual Private Cloud (VPC) that the domain uses for
    /// communication.
    /// 
    /// Regex Pattern: `^[-0-9a-zA-Z]+$`
    #[serde(rename = "vpcID")]
    pub vpc_id: String,
}

/// The default settings to use to create a user profile when UserSettings isn't
/// specified in the call to the CreateUserProfile API.
/// 
/// SecurityGroups is aggregated when specified in both calls. For all other
/// settings in UserSettings, the values specified in CreateUserProfile take
/// precedence over those specified in CreateDomain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettings {
    /// The Code Editor application settings.
    /// 
    /// For more information about Code Editor, see Get started with Code Editor
    /// in Amazon SageMaker (https://docs.aws.amazon.com/sagemaker/latest/dg/code-editor.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "codeEditorAppSettings")]
    pub code_editor_app_settings: Option<DomainDefaultUserSettingsCodeEditorAppSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customFileSystemConfigs")]
    pub custom_file_system_configs: Option<Vec<DomainDefaultUserSettingsCustomFileSystemConfigs>>,
    /// Details about the POSIX identity that is used for file system operations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customPosixUserConfig")]
    pub custom_posix_user_config: Option<DomainDefaultUserSettingsCustomPosixUserConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultLandingURI")]
    pub default_landing_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionRole")]
    pub execution_role: Option<String>,
    /// The settings for the JupyterLab application.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jupyterLabAppSettings")]
    pub jupyter_lab_app_settings: Option<DomainDefaultUserSettingsJupyterLabAppSettings>,
    /// The JupyterServer app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jupyterServerAppSettings")]
    pub jupyter_server_app_settings: Option<DomainDefaultUserSettingsJupyterServerAppSettings>,
    /// The KernelGateway app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kernelGatewayAppSettings")]
    pub kernel_gateway_app_settings: Option<DomainDefaultUserSettingsKernelGatewayAppSettings>,
    /// A collection of settings that configure user interaction with the RStudioServerPro
    /// app.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rStudioServerProAppSettings")]
    pub r_studio_server_pro_app_settings: Option<DomainDefaultUserSettingsRStudioServerProAppSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroups")]
    pub security_groups: Option<Vec<String>>,
    /// Specifies options for sharing Amazon SageMaker Studio notebooks. These settings
    /// are specified as part of DefaultUserSettings when the CreateDomain API is
    /// called, and as part of UserSettings when the CreateUserProfile API is called.
    /// When SharingSettings is not specified, notebook sharing isn't allowed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharingSettings")]
    pub sharing_settings: Option<DomainDefaultUserSettingsSharingSettings>,
    /// The default storage settings for a space.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "spaceStorageSettings")]
    pub space_storage_settings: Option<DomainDefaultUserSettingsSpaceStorageSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "studioWebPortal")]
    pub studio_web_portal: Option<String>,
    /// The TensorBoard app settings.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tensorBoardAppSettings")]
    pub tensor_board_app_settings: Option<DomainDefaultUserSettingsTensorBoardAppSettings>,
}

/// The Code Editor application settings.
/// 
/// For more information about Code Editor, see Get started with Code Editor
/// in Amazon SageMaker (https://docs.aws.amazon.com/sagemaker/latest/dg/code-editor.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsCodeEditorAppSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDefaultUserSettingsCodeEditorAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsCodeEditorAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
}

/// The settings for assigning a custom file system to a user profile or space
/// for an Amazon SageMaker Domain. Permitted users can access this file system
/// in Amazon SageMaker Studio.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsCustomFileSystemConfigs {
    /// The settings for assigning a custom Amazon EFS file system to a user profile
    /// or space for an Amazon SageMaker Domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "efsFileSystemConfig")]
    pub efs_file_system_config: Option<DomainDefaultUserSettingsCustomFileSystemConfigsEfsFileSystemConfig>,
}

/// The settings for assigning a custom Amazon EFS file system to a user profile
/// or space for an Amazon SageMaker Domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsCustomFileSystemConfigsEfsFileSystemConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemID")]
    pub file_system_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileSystemPath")]
    pub file_system_path: Option<String>,
}

/// Details about the POSIX identity that is used for file system operations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsCustomPosixUserConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}

/// The settings for the JupyterLab application.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsJupyterLabAppSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customImages")]
    pub custom_images: Option<Vec<DomainDefaultUserSettingsJupyterLabAppSettingsCustomImages>>,
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDefaultUserSettingsJupyterLabAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// A custom SageMaker image. For more information, see Bring your own SageMaker
/// image (https://docs.aws.amazon.com/sagemaker/latest/dg/studio-byoi.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsJupyterLabAppSettingsCustomImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appImageConfigName")]
    pub app_image_config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageName")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageVersionNumber")]
    pub image_version_number: Option<i64>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsJupyterLabAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
}

/// The JupyterServer app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsJupyterServerAppSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDefaultUserSettingsJupyterServerAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsJupyterServerAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
}

/// The KernelGateway app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsKernelGatewayAppSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customImages")]
    pub custom_images: Option<Vec<DomainDefaultUserSettingsKernelGatewayAppSettingsCustomImages>>,
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDefaultUserSettingsKernelGatewayAppSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARNs")]
    pub lifecycle_config_ar_ns: Option<Vec<String>>,
}

/// A custom SageMaker image. For more information, see Bring your own SageMaker
/// image (https://docs.aws.amazon.com/sagemaker/latest/dg/studio-byoi.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsKernelGatewayAppSettingsCustomImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appImageConfigName")]
    pub app_image_config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageName")]
    pub image_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageVersionNumber")]
    pub image_version_number: Option<i64>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsKernelGatewayAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
}

/// A collection of settings that configure user interaction with the RStudioServerPro
/// app.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsRStudioServerProAppSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessStatus")]
    pub access_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userGroup")]
    pub user_group: Option<String>,
}

/// Specifies options for sharing Amazon SageMaker Studio notebooks. These settings
/// are specified as part of DefaultUserSettings when the CreateDomain API is
/// called, and as part of UserSettings when the CreateUserProfile API is called.
/// When SharingSettings is not specified, notebook sharing isn't allowed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsSharingSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notebookOutputOption")]
    pub notebook_output_option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3KMSKeyID")]
    pub s3_kms_key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3OutputPath")]
    pub s3_output_path: Option<String>,
}

/// The default storage settings for a space.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsSpaceStorageSettings {
    /// A collection of default EBS storage settings that apply to spaces created
    /// within a domain or user profile.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEBSStorageSettings")]
    pub default_ebs_storage_settings: Option<DomainDefaultUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings>,
}

/// A collection of default EBS storage settings that apply to spaces created
/// within a domain or user profile.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsSpaceStorageSettingsDefaultEbsStorageSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEBSVolumeSizeInGb")]
    pub default_ebs_volume_size_in_gb: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maximumEBSVolumeSizeInGb")]
    pub maximum_ebs_volume_size_in_gb: Option<i64>,
}

/// The TensorBoard app settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsTensorBoardAppSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDefaultUserSettingsTensorBoardAppSettingsDefaultResourceSpec>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDefaultUserSettingsTensorBoardAppSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
}

/// A collection of Domain settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDomainSettings {
    /// A collection of settings that configure the domain's Docker interaction.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dockerSettings")]
    pub docker_settings: Option<DomainDomainSettingsDockerSettings>,
    /// A collection of settings that configure the RStudioServerPro Domain-level
    /// app.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rStudioServerProDomainSettings")]
    pub r_studio_server_pro_domain_settings: Option<DomainDomainSettingsRStudioServerProDomainSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
}

/// A collection of settings that configure the domain's Docker interaction.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDomainSettingsDockerSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableDockerAccess")]
    pub enable_docker_access: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcOnlyTrustedAccounts")]
    pub vpc_only_trusted_accounts: Option<Vec<String>>,
}

/// A collection of settings that configure the RStudioServerPro Domain-level
/// app.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDomainSettingsRStudioServerProDomainSettings {
    /// Specifies the ARN's of a SageMaker image and SageMaker image version, and
    /// the instance type that the version runs on.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultResourceSpec")]
    pub default_resource_spec: Option<DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainExecutionRoleARN")]
    pub domain_execution_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rStudioConnectURL")]
    pub r_studio_connect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rStudioPackageManagerURL")]
    pub r_studio_package_manager_url: Option<String>,
}

/// Specifies the ARN's of a SageMaker image and SageMaker image version, and
/// the instance type that the version runs on.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDomainSettingsRStudioServerProDomainSettingsDefaultResourceSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigARN")]
    pub lifecycle_config_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageARN")]
    pub sage_maker_image_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionARN")]
    pub sage_maker_image_version_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sageMakerImageVersionAlias")]
    pub sage_maker_image_version_alias: Option<String>,
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
pub struct DomainTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DomainStatus defines the observed state of Domain
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DomainStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The domain ID.
    /// 
    /// Regex Pattern: `^d-(-*[a-z0-9]){1,61}$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainID")]
    pub domain_id: Option<String>,
    /// The status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The URL to the created domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatusAckResourceMetadata {
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

