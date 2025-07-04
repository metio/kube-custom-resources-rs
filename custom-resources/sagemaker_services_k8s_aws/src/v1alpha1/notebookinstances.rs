// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sagemaker-controller/sagemaker.services.k8s.aws/v1alpha1/notebookinstances.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// NotebookInstanceSpec defines the desired state of NotebookInstance.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sagemaker.services.k8s.aws", version = "v1alpha1", kind = "NotebookInstance", plural = "notebookinstances")]
#[kube(namespaced)]
#[kube(status = "NotebookInstanceStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NotebookInstanceSpec {
    /// This parameter is no longer supported. Elastic Inference (EI) is no longer
    /// available.
    /// 
    /// This parameter was used to specify a list of EI instance types to associate
    /// with this notebook instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "acceleratorTypes")]
    pub accelerator_types: Option<Vec<String>>,
    /// An array of up to three Git repositories to associate with the notebook instance.
    /// These can be either the names of Git repositories stored as resources in
    /// your account, or the URL of Git repositories in Amazon Web Services CodeCommit
    /// (https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or
    /// in any other Git repository. These repositories are cloned at the same level
    /// as the default repository of your notebook instance. For more information,
    /// see Associating Git Repositories with SageMaker Notebook Instances (https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalCodeRepositories")]
    pub additional_code_repositories: Option<Vec<String>>,
    /// A Git repository to associate with the notebook instance as its default code
    /// repository. This can be either the name of a Git repository stored as a resource
    /// in your account, or the URL of a Git repository in Amazon Web Services CodeCommit
    /// (https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html) or
    /// in any other Git repository. When you open a notebook instance, it opens
    /// in the directory that contains this repository. For more information, see
    /// Associating Git Repositories with SageMaker Notebook Instances (https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html).
    /// 
    /// Regex Pattern: `^https://([^/]+)/?(.*)$|^[a-zA-Z0-9](-*[a-zA-Z0-9])*$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultCodeRepository")]
    pub default_code_repository: Option<String>,
    /// Sets whether SageMaker provides internet access to the notebook instance.
    /// If you set this to Disabled this notebook instance is able to access resources
    /// only in your VPC, and is not be able to connect to SageMaker training and
    /// endpoint services unless you configure a NAT Gateway in your VPC.
    /// 
    /// For more information, see Notebook Instances Are Internet-Enabled by Default
    /// (https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access).
    /// You can set the value of this parameter to Disabled only if you set a value
    /// for the SubnetId parameter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "directInternetAccess")]
    pub direct_internet_access: Option<String>,
    /// The type of ML compute instance to launch for the notebook instance.
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// The Amazon Resource Name (ARN) of a Amazon Web Services Key Management Service
    /// key that SageMaker uses to encrypt data on the storage volume attached to
    /// your notebook instance. The KMS key you provide must be enabled. For information,
    /// see Enabling and Disabling Keys (https://docs.aws.amazon.com/kms/latest/developerguide/enabling-keys.html)
    /// in the Amazon Web Services Key Management Service Developer Guide.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9:/_-]*$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// The name of a lifecycle configuration to associate with the notebook instance.
    /// For information about lifestyle configurations, see Step 2.1: (Optional)
    /// Customize a Notebook Instance (https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html).
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9](-*[a-zA-Z0-9])*$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifecycleConfigName")]
    pub lifecycle_config_name: Option<String>,
    /// The name of the new notebook instance.
    /// 
    /// Regex Pattern: `^[a-zA-Z0-9](-*[a-zA-Z0-9])*$`
    #[serde(rename = "notebookInstanceName")]
    pub notebook_instance_name: String,
    /// The platform identifier of the notebook instance runtime environment.
    /// 
    /// Regex Pattern: `^(notebook-al1-v1|notebook-al2-v1|notebook-al2-v2|notebook-al2-v3)$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "platformIdentifier")]
    pub platform_identifier: Option<String>,
    /// When you send any requests to Amazon Web Services resources from the notebook
    /// instance, SageMaker assumes this role to perform tasks on your behalf. You
    /// must grant this role necessary permissions so SageMaker can perform these
    /// tasks. The policy must allow the SageMaker service principal (sagemaker.amazonaws.com)
    /// permissions to assume this role. For more information, see SageMaker Roles
    /// (https://docs.aws.amazon.com/sagemaker/latest/dg/sagemaker-roles.html).
    /// 
    /// To be able to pass this role to SageMaker, the caller of this API must have
    /// the iam:PassRole permission.
    /// 
    /// Regex Pattern: `^arn:aws[a-z\-]*:iam::\d{12}:role/?[a-zA-Z_0-9+=,.@\-_/]+$`
    #[serde(rename = "roleARN")]
    pub role_arn: String,
    /// Whether root access is enabled or disabled for users of the notebook instance.
    /// The default value is Enabled.
    /// 
    /// Lifecycle configurations need root access to be able to set up a notebook
    /// instance. Because of this, lifecycle configurations associated with a notebook
    /// instance always run with root access even if you disable root access for
    /// users.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootAccess")]
    pub root_access: Option<String>,
    /// The VPC security group IDs, in the form sg-xxxxxxxx. The security groups
    /// must be for the same VPC as specified in the subnet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    /// The ID of the subnet in a VPC to which you would like to have a connectivity
    /// from your ML compute instance.
    /// 
    /// Regex Pattern: `^[-0-9a-zA-Z]+$`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetID")]
    pub subnet_id: Option<String>,
    /// An array of key-value pairs. You can use tags to categorize your Amazon Web
    /// Services resources in different ways, for example, by purpose, owner, or
    /// environment. For more information, see Tagging Amazon Web Services Resources
    /// (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<NotebookInstanceTags>>,
    /// The size, in GB, of the ML storage volume to attach to the notebook instance.
    /// The default value is 5 GB.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSizeInGB")]
    pub volume_size_in_gb: Option<i64>,
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
pub struct NotebookInstanceTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// NotebookInstanceStatus defines the observed state of NotebookInstance
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotebookInstanceStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<NotebookInstanceStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// If status is Failed, the reason it failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// The status of the notebook instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notebookInstanceStatus")]
    pub notebook_instance_status: Option<String>,
    /// The URL that you use to connect to the Jupyter notebook that is running in
    /// your notebook instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stoppedByControllerMetadata")]
    pub stopped_by_controller_metadata: Option<String>,
    /// The URL that you use to connect to the Jupyter notebook that is running in
    /// your notebook instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NotebookInstanceStatusAckResourceMetadata {
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

