// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/rds-controller/rds.services.k8s.aws/v1alpha1/dbproxies.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DBProxySpec defines the desired state of DBProxy.
/// 
/// The data structure representing a proxy managed by the RDS Proxy.
/// 
/// This data type is used as a response element in the DescribeDBProxies action.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rds.services.k8s.aws", version = "v1alpha1", kind = "DBProxy", plural = "dbproxies")]
#[kube(namespaced)]
#[kube(status = "DBProxyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DBProxySpec {
    /// The authorization mechanism that the proxy uses.
    pub auth: Vec<DBProxyAuth>,
    /// Whether the proxy includes detailed information about SQL statements in its
    /// logs. This information helps you to debug issues involving SQL behavior or
    /// the performance and scalability of the proxy connections. The debug information
    /// includes the text of SQL statements that you submit through the proxy. Thus,
    /// only enable this setting when needed for debugging, and only when you have
    /// security measures in place to safeguard any sensitive information that appears
    /// in the logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "debugLogging")]
    pub debug_logging: Option<bool>,
    /// The kinds of databases that the proxy can connect to. This value determines
    /// which database network protocol the proxy recognizes when it interprets network
    /// traffic to and from the database. For Aurora MySQL, RDS for MariaDB, and
    /// RDS for MySQL databases, specify MYSQL. For Aurora PostgreSQL and RDS for
    /// PostgreSQL databases, specify POSTGRESQL. For RDS for Microsoft SQL Server,
    /// specify SQLSERVER.
    #[serde(rename = "engineFamily")]
    pub engine_family: String,
    /// The number of seconds that a connection to the proxy can be inactive before
    /// the proxy disconnects it. You can set this value higher or lower than the
    /// connection timeout limit for the associated database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleClientTimeout")]
    pub idle_client_timeout: Option<i64>,
    /// The identifier for the proxy. This name must be unique for all proxies owned
    /// by your Amazon Web Services account in the specified Amazon Web Services
    /// Region. An identifier must begin with a letter and must contain only ASCII
    /// letters, digits, and hyphens; it can't end with a hyphen or contain two consecutive
    /// hyphens.
    pub name: String,
    /// A Boolean parameter that specifies whether Transport Layer Security (TLS)
    /// encryption is required for connections to the proxy. By enabling this setting,
    /// you can enforce encrypted TLS connections to the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requireTLS")]
    pub require_tls: Option<bool>,
    /// The Amazon Resource Name (ARN) of the IAM role that the proxy uses to access
    /// secrets in Amazon Web Services Secrets Manager.
    #[serde(rename = "roleARN")]
    pub role_arn: String,
    /// An optional set of key-value pairs to associate arbitrary data of your choosing
    /// with the proxy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DBProxyTags>>,
    /// One or more VPC security group IDs to associate with the new proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcSecurityGroupIDs")]
    pub vpc_security_group_i_ds: Option<Vec<String>>,
    /// One or more VPC subnet IDs to associate with the new proxy.
    #[serde(rename = "vpcSubnetIDs")]
    pub vpc_subnet_i_ds: Vec<String>,
}

/// Specifies the details of authentication used by a proxy to log in as a specific
/// database user.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBProxyAuth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authScheme")]
    pub auth_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientPasswordAuthType")]
    pub client_password_auth_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iamAuth")]
    pub iam_auth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretARN")]
    pub secret_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
}

/// Metadata assigned to an Amazon RDS resource consisting of a key-value pair.
/// 
/// For more information, see Tagging Amazon RDS Resources (https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html)
/// in the Amazon RDS User Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBProxyTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// DBProxyStatus defines the observed state of DBProxy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBProxyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DBProxyStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The date and time when the proxy was first created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdDate")]
    pub created_date: Option<String>,
    /// The endpoint that you can use to connect to the DB proxy. You include the
    /// endpoint value in the connection string for a database client application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The current status of this proxy. A status of available means the proxy is
    /// ready to handle requests. Other values indicate that you must wait for the
    /// proxy to be ready, or take some action to resolve an issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The date and time when the proxy was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedDate")]
    pub updated_date: Option<String>,
    /// Provides the VPC ID of the DB proxy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DBProxyStatusAckResourceMetadata {
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

