// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/openshift/hive/hive.openshift.io/v1/dnszones.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DNSZoneSpec defines the desired state of DNSZone
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hive.openshift.io", version = "v1", kind = "DNSZone", plural = "dnszones")]
#[kube(namespaced)]
#[kube(status = "DNSZoneStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DNSZoneSpec {
    /// AWS specifies AWS-specific cloud configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<DNSZoneAws>,
    /// Azure specifes Azure-specific cloud configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<DNSZoneAzure>,
    /// GCP specifies GCP-specific cloud configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcp: Option<DNSZoneGcp>,
    /// LinkToParentDomain specifies whether DNS records should
    /// be automatically created to link this DNSZone with a
    /// parent domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkToParentDomain")]
    pub link_to_parent_domain: Option<bool>,
    /// PreserveOnDelete allows the user to disconnect a DNSZone from Hive without deprovisioning it.
    /// This can also be used to abandon ongoing DNSZone deprovision.
    /// Typically set automatically due to PreserveOnDelete being set on a ClusterDeployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preserveOnDelete")]
    pub preserve_on_delete: Option<bool>,
    /// Zone is the DNS zone to host
    pub zone: String,
}

/// AWS specifies AWS-specific cloud configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAws {
    /// AdditionalTags is a set of additional tags to set on the DNS hosted zone. In addition
    /// to these tags,the DNS Zone controller will set a hive.openhsift.io/hostedzone tag
    /// identifying the HostedZone record that it belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalTags")]
    pub additional_tags: Option<Vec<DNSZoneAwsAdditionalTags>>,
    /// CredentialsAssumeRole refers to the IAM role that must be assumed to obtain
    /// AWS account access for the DNS CRUD operations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsAssumeRole")]
    pub credentials_assume_role: Option<DNSZoneAwsCredentialsAssumeRole>,
    /// CredentialsSecretRef contains a reference to a secret that contains AWS credentials
    /// for CRUD operations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialsSecretRef")]
    pub credentials_secret_ref: Option<DNSZoneAwsCredentialsSecretRef>,
    /// Region is the AWS region to use for route53 operations.
    /// This defaults to us-east-1.
    /// For AWS China, use cn-northwest-1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// AWSResourceTag represents a tag that is applied to an AWS cloud resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAwsAdditionalTags {
    /// Key is the key for the tag
    pub key: String,
    /// Value is the value for the tag
    pub value: String,
}

/// CredentialsAssumeRole refers to the IAM role that must be assumed to obtain
/// AWS account access for the DNS CRUD operations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAwsCredentialsAssumeRole {
    /// ExternalID is random string generated by platform so that assume role
    /// is protected from confused deputy problem.
    /// more info: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalID")]
    pub external_id: Option<String>,
    #[serde(rename = "roleARN")]
    pub role_arn: String,
}

/// CredentialsSecretRef contains a reference to a secret that contains AWS credentials
/// for CRUD operations
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAwsCredentialsSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Azure specifes Azure-specific cloud configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAzure {
    /// CloudName is the name of the Azure cloud environment which can be used to configure the Azure SDK
    /// with the appropriate Azure API endpoints.
    /// If empty, the value is equal to "AzurePublicCloud".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudName")]
    pub cloud_name: Option<DNSZoneAzureCloudName>,
    /// CredentialsSecretRef references a secret that will be used to authenticate with
    /// Azure CloudDNS. It will need permission to create and manage CloudDNS Hosted Zones.
    /// Secret should have a key named 'osServicePrincipal.json'.
    /// The credentials must specify the project to use.
    #[serde(rename = "credentialsSecretRef")]
    pub credentials_secret_ref: DNSZoneAzureCredentialsSecretRef,
    /// ResourceGroupName specifies the Azure resource group in which the Hosted Zone should be created.
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

/// Azure specifes Azure-specific cloud configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSZoneAzureCloudName {
    #[serde(rename = "")]
    KopiumEmpty,
    AzurePublicCloud,
    #[serde(rename = "AzureUSGovernmentCloud")]
    AzureUsGovernmentCloud,
    AzureChinaCloud,
    AzureGermanCloud,
}

/// CredentialsSecretRef references a secret that will be used to authenticate with
/// Azure CloudDNS. It will need permission to create and manage CloudDNS Hosted Zones.
/// Secret should have a key named 'osServicePrincipal.json'.
/// The credentials must specify the project to use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneAzureCredentialsSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// GCP specifies GCP-specific cloud configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneGcp {
    /// CredentialsSecretRef references a secret that will be used to authenticate with
    /// GCP CloudDNS. It will need permission to create and manage CloudDNS Hosted Zones.
    /// Secret should have a key named 'osServiceAccount.json'.
    /// The credentials must specify the project to use.
    #[serde(rename = "credentialsSecretRef")]
    pub credentials_secret_ref: DNSZoneGcpCredentialsSecretRef,
}

/// CredentialsSecretRef references a secret that will be used to authenticate with
/// GCP CloudDNS. It will need permission to create and manage CloudDNS Hosted Zones.
/// Secret should have a key named 'osServiceAccount.json'.
/// The credentials must specify the project to use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneGcpCredentialsSecretRef {
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// DNSZoneStatus defines the observed state of DNSZone
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneStatus {
    /// AWSDNSZoneStatus contains status information specific to AWS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<DNSZoneStatusAws>,
    /// AzureDNSZoneStatus contains status information specific to Azure
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azure: Option<DNSZoneStatusAzure>,
    /// Conditions includes more detailed status for the DNSZone
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// GCPDNSZoneStatus contains status information specific to GCP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcp: Option<DNSZoneStatusGcp>,
    /// LastSyncGeneration is the generation of the zone resource that was last sync'd. This is used to know
    /// if the Object has changed and we should sync immediately.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncGeneration")]
    pub last_sync_generation: Option<i64>,
    /// LastSyncTimestamp is the time that the zone was last sync'd.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncTimestamp")]
    pub last_sync_timestamp: Option<String>,
    /// NameServers is a list of nameservers for this DNS zone
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameServers")]
    pub name_servers: Option<Vec<String>>,
}

/// AWSDNSZoneStatus contains status information specific to AWS
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneStatusAws {
    /// ZoneID is the ID of the zone in AWS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneID")]
    pub zone_id: Option<String>,
}

/// AzureDNSZoneStatus contains status information specific to Azure
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneStatusAzure {
}

/// GCPDNSZoneStatus contains status information specific to GCP
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSZoneStatusGcp {
    /// ZoneName is the name of the zone in GCP Cloud DNS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneName")]
    pub zone_name: Option<String>,
}

