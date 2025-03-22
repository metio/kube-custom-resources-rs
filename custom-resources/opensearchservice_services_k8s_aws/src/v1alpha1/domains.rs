// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/opensearchservice-controller/opensearchservice.services.k8s.aws/v1alpha1/domains.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DomainSpec defines the desired state of Domain.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "opensearchservice.services.k8s.aws", version = "v1alpha1", kind = "Domain", plural = "domains")]
#[kube(namespaced)]
#[kube(status = "DomainStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DomainSpec {
    /// Identity and Access Management (IAM) policy document specifying the access
    /// policies for the new domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessPolicies")]
    pub access_policies: Option<String>,
    /// Key-value pairs to specify advanced configuration options. The following
    /// key-value pairs are supported:
    /// 
    ///    * "rest.action.multi.allow_explicit_index": "true" | "false" - Note the
    ///    use of a string rather than a boolean. Specifies whether explicit references
    ///    to indexes are allowed inside the body of HTTP requests. If you want to
    ///    configure access policies for domain sub-resources, such as specific indexes
    ///    and domain APIs, you must disable this property. Default is true.
    /// 
    ///    * "indices.fielddata.cache.size": "80" - Note the use of a string rather
    ///    than a boolean. Specifies the percentage of heap space allocated to field
    ///    data. Default is unbounded.
    /// 
    ///    * "indices.query.bool.max_clause_count": "1024" - Note the use of a string
    ///    rather than a boolean. Specifies the maximum number of clauses allowed
    ///    in a Lucene boolean query. Default is 1,024. Queries with more than the
    ///    permitted number of clauses result in a TooManyClauses error.
    /// 
    ///    * "override_main_response_version": "true" | "false" - Note the use of
    ///    a string rather than a boolean. Specifies whether the domain reports its
    ///    version as 7.10 to allow Elasticsearch OSS clients and plugins to continue
    ///    working with it. Default is false when creating a domain and true when
    ///    upgrading a domain.
    /// 
    /// For more information, see Advanced cluster parameters (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomain-configure-advanced-options).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "advancedOptions")]
    pub advanced_options: Option<BTreeMap<String, String>>,
    /// Options for fine-grained access control.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "advancedSecurityOptions")]
    pub advanced_security_options: Option<DomainAdvancedSecurityOptions>,
    /// Options for all machine learning features for the specified domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "aimlOptions")]
    pub aiml_options: Option<DomainAimlOptions>,
    /// Options for Auto-Tune.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoTuneOptions")]
    pub auto_tune_options: Option<DomainAutoTuneOptions>,
    /// Container for the cluster configuration of a domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterConfig")]
    pub cluster_config: Option<DomainClusterConfig>,
    /// Key-value pairs to configure Amazon Cognito authentication. For more information,
    /// see Configuring Amazon Cognito authentication for OpenSearch Dashboards (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cognitoOptions")]
    pub cognito_options: Option<DomainCognitoOptions>,
    /// Additional options for the domain endpoint, such as whether to require HTTPS
    /// for all traffic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainEndpointOptions")]
    pub domain_endpoint_options: Option<DomainDomainEndpointOptions>,
    /// Container for the parameters required to enable EBS-based storage for an
    /// OpenSearch Service domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ebsOptions")]
    pub ebs_options: Option<DomainEbsOptions>,
    /// Key-value pairs to enable encryption at rest.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "encryptionAtRestOptions")]
    pub encryption_at_rest_options: Option<DomainEncryptionAtRestOptions>,
    /// String of format Elasticsearch_X.Y or OpenSearch_X.Y to specify the engine
    /// version for the OpenSearch Service domain. For example, OpenSearch_1.0 or
    /// Elasticsearch_7.9. For more information, see Creating and managing Amazon
    /// OpenSearch Service domains (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/createupdatedomains.html#createdomains).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "engineVersion")]
    pub engine_version: Option<String>,
    /// Specify either dual stack or IPv4 as your IP address type. Dual stack allows
    /// you to share domain resources across IPv4 and IPv6 address types, and is
    /// the recommended option. If you set your IP address type to dual stack, you
    /// can't change your address type later.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddressType")]
    pub ip_address_type: Option<String>,
    /// Key-value pairs to configure log publishing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logPublishingOptions")]
    pub log_publishing_options: Option<BTreeMap<String, DomainLogPublishingOptions>>,
    /// Name of the OpenSearch Service domain to create. Domain names are unique
    /// across the domains owned by an account within an Amazon Web Services Region.
    pub name: String,
    /// Enables node-to-node encryption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeToNodeEncryptionOptions")]
    pub node_to_node_encryption_options: Option<DomainNodeToNodeEncryptionOptions>,
    /// Specifies a daily 10-hour time block during which OpenSearch Service can
    /// perform configuration changes on the domain, including service software updates
    /// and Auto-Tune enhancements that require a blue/green deployment. If no options
    /// are specified, the default start time of 10:00 P.M. local time (for the Region
    /// that the domain is created in) is used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offPeakWindowOptions")]
    pub off_peak_window_options: Option<DomainOffPeakWindowOptions>,
    /// Software update options for the domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "softwareUpdateOptions")]
    pub software_update_options: Option<DomainSoftwareUpdateOptions>,
    /// List of tags to add to the domain upon creation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DomainTags>>,
    /// Container for the values required to configure VPC access domains. If you
    /// don't specify these values, OpenSearch Service creates the domain with a
    /// public endpoint. For more information, see Launching your Amazon OpenSearch
    /// Service domains using a VPC (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcOptions")]
    pub vpc_options: Option<DomainVpcOptions>,
}

/// Options for fine-grained access control.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "anonymousAuthEnabled")]
    pub anonymous_auth_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "internalUserDatabaseEnabled")]
    pub internal_user_database_enabled: Option<bool>,
    /// The JWT authentication and authorization configuration for an Amazon OpenSearch
    /// Service domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtOptions")]
    pub jwt_options: Option<DomainAdvancedSecurityOptionsJwtOptions>,
    /// Credentials for the master user for a domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserOptions")]
    pub master_user_options: Option<DomainAdvancedSecurityOptionsMasterUserOptions>,
    /// The SAML authentication configuration for an Amazon OpenSearch Service domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sAMLOptions")]
    pub s_aml_options: Option<DomainAdvancedSecurityOptionsSAmlOptions>,
}

/// The JWT authentication and authorization configuration for an Amazon OpenSearch
/// Service domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptionsJwtOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "publicKey")]
    pub public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolesKey")]
    pub roles_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectKey")]
    pub subject_key: Option<String>,
}

/// Credentials for the master user for a domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptionsMasterUserOptions {
    /// The Amazon Resource Name (ARN) of the domain. See Identifiers for IAM Entities
    /// (https://docs.aws.amazon.com/IAM/latest/UserGuide/index.html) in Using Amazon
    /// Web Services Identity and Access Management for more information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserARN")]
    pub master_user_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserName")]
    pub master_user_name: Option<String>,
    /// SecretKeyReference combines a k8s corev1.SecretReference with a
    /// specific key within the referred-to Secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserPassword")]
    pub master_user_password: Option<DomainAdvancedSecurityOptionsMasterUserOptionsMasterUserPassword>,
}

/// SecretKeyReference combines a k8s corev1.SecretReference with a
/// specific key within the referred-to Secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptionsMasterUserOptionsMasterUserPassword {
    /// Key is the key within the secret
    pub key: String,
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The SAML authentication configuration for an Amazon OpenSearch Service domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptionsSAmlOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The SAML identity povider information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idp: Option<DomainAdvancedSecurityOptionsSAmlOptionsIdp>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterBackendRole")]
    pub master_backend_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterUserName")]
    pub master_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolesKey")]
    pub roles_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionTimeoutMinutes")]
    pub session_timeout_minutes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectKey")]
    pub subject_key: Option<String>,
}

/// The SAML identity povider information.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAdvancedSecurityOptionsSAmlOptionsIdp {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "entityID")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metadataContent")]
    pub metadata_content: Option<String>,
}

/// Options for all machine learning features for the specified domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAimlOptions {
    /// Container for parameters required to enable the natural language query generation
    /// feature.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "naturalLanguageQueryGenerationOptions")]
    pub natural_language_query_generation_options: Option<DomainAimlOptionsNaturalLanguageQueryGenerationOptions>,
}

/// Container for parameters required to enable the natural language query generation
/// feature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAimlOptionsNaturalLanguageQueryGenerationOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredState")]
    pub desired_state: Option<String>,
}

/// Options for Auto-Tune.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAutoTuneOptions {
    /// The Auto-Tune desired state. Valid values are ENABLED and DISABLED.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredState")]
    pub desired_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maintenanceSchedules")]
    pub maintenance_schedules: Option<Vec<DomainAutoTuneOptionsMaintenanceSchedules>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useOffPeakWindow")]
    pub use_off_peak_window: Option<bool>,
}

/// This object is deprecated. Use the domain's off-peak window (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html)
/// to schedule Auto-Tune optimizations. For migration instructions, see Migrating
/// from Auto-Tune maintenance windows (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html#off-peak-migrate).
/// 
/// The Auto-Tune maintenance schedule. For more information, see Auto-Tune for
/// Amazon OpenSearch Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/auto-tune.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAutoTuneOptionsMaintenanceSchedules {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cronExpressionForRecurrence")]
    pub cron_expression_for_recurrence: Option<String>,
    /// The duration of a maintenance schedule. For more information, see Auto-Tune
    /// for Amazon OpenSearch Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/auto-tune.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DomainAutoTuneOptionsMaintenanceSchedulesDuration>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startAt")]
    pub start_at: Option<String>,
}

/// The duration of a maintenance schedule. For more information, see Auto-Tune
/// for Amazon OpenSearch Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/auto-tune.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainAutoTuneOptionsMaintenanceSchedulesDuration {
    /// The unit of a maintenance schedule duration. Valid value is HOUR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Integer that specifies the value of a maintenance schedule duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

/// Container for the cluster configuration of a domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainClusterConfig {
    /// Container for the parameters required to enable cold storage for an OpenSearch
    /// Service domain. For more information, see Cold storage for Amazon OpenSearch
    /// Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cold-storage.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "coldStorageOptions")]
    pub cold_storage_options: Option<DomainClusterConfigColdStorageOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedMasterCount")]
    pub dedicated_master_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedMasterEnabled")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dedicatedMasterType")]
    pub dedicated_master_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceCount")]
    pub instance_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceType")]
    pub instance_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiAZWithStandbyEnabled")]
    pub multi_az_with_standby_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "warmCount")]
    pub warm_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "warmEnabled")]
    pub warm_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "warmType")]
    pub warm_type: Option<String>,
    /// The zone awareness configuration for an Amazon OpenSearch Service domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneAwarenessConfig")]
    pub zone_awareness_config: Option<DomainClusterConfigZoneAwarenessConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "zoneAwarenessEnabled")]
    pub zone_awareness_enabled: Option<bool>,
}

/// Container for the parameters required to enable cold storage for an OpenSearch
/// Service domain. For more information, see Cold storage for Amazon OpenSearch
/// Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cold-storage.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainClusterConfigColdStorageOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// The zone awareness configuration for an Amazon OpenSearch Service domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainClusterConfigZoneAwarenessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "availabilityZoneCount")]
    pub availability_zone_count: Option<i64>,
}

/// Key-value pairs to configure Amazon Cognito authentication. For more information,
/// see Configuring Amazon Cognito authentication for OpenSearch Dashboards (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/cognito-auth.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainCognitoOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityPoolID")]
    pub identity_pool_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userPoolID")]
    pub user_pool_id: Option<String>,
}

/// Additional options for the domain endpoint, such as whether to require HTTPS
/// for all traffic.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainDomainEndpointOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customEndpoint")]
    pub custom_endpoint: Option<String>,
    /// The Amazon Resource Name (ARN) of the domain. See Identifiers for IAM Entities
    /// (https://docs.aws.amazon.com/IAM/latest/UserGuide/index.html) in Using Amazon
    /// Web Services Identity and Access Management for more information.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customEndpointCertificateARN")]
    pub custom_endpoint_certificate_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customEndpointEnabled")]
    pub custom_endpoint_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enforceHTTPS")]
    pub enforce_https: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecurityPolicy")]
    pub tls_security_policy: Option<String>,
}

/// Container for the parameters required to enable EBS-based storage for an
/// OpenSearch Service domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainEbsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ebsEnabled")]
    pub ebs_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeSize")]
    pub volume_size: Option<i64>,
    /// The type of EBS volume that a domain uses. For more information, see Configuring
    /// EBS-based storage (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/opensearch-createupdatedomains.html#opensearch-createdomain-configure-ebs).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeType")]
    pub volume_type: Option<String>,
}

/// Key-value pairs to enable encryption at rest.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainEncryptionAtRestOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
}

/// Key-value pairs to configure log publishing.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainLogPublishingOptions {
    /// ARN of the Cloudwatch log group to publish logs to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloudWatchLogsLogGroupARN")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Enables node-to-node encryption.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainNodeToNodeEncryptionOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Specifies a daily 10-hour time block during which OpenSearch Service can
/// perform configuration changes on the domain, including service software updates
/// and Auto-Tune enhancements that require a blue/green deployment. If no options
/// are specified, the default start time of 10:00 P.M. local time (for the Region
/// that the domain is created in) is used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainOffPeakWindowOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A custom 10-hour, low-traffic window during which OpenSearch Service can
    /// perform mandatory configuration changes on the domain. These actions can
    /// include scheduled service software updates and blue/green Auto-Tune enhancements.
    /// OpenSearch Service will schedule these actions during the window that you
    /// specify.
    /// 
    /// If you don't specify a window start time, it defaults to 10:00 P.M. local
    /// time.
    /// 
    /// For more information, see Defining off-peak maintenance windows for Amazon
    /// OpenSearch Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offPeakWindow")]
    pub off_peak_window: Option<DomainOffPeakWindowOptionsOffPeakWindow>,
}

/// A custom 10-hour, low-traffic window during which OpenSearch Service can
/// perform mandatory configuration changes on the domain. These actions can
/// include scheduled service software updates and blue/green Auto-Tune enhancements.
/// OpenSearch Service will schedule these actions during the window that you
/// specify.
/// 
/// If you don't specify a window start time, it defaults to 10:00 P.M. local
/// time.
/// 
/// For more information, see Defining off-peak maintenance windows for Amazon
/// OpenSearch Service (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/off-peak.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainOffPeakWindowOptionsOffPeakWindow {
    /// The desired start time for an off-peak maintenance window (https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_OffPeakWindow.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "windowStartTime")]
    pub window_start_time: Option<DomainOffPeakWindowOptionsOffPeakWindowWindowStartTime>,
}

/// The desired start time for an off-peak maintenance window (https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_OffPeakWindow.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainOffPeakWindowOptionsOffPeakWindowWindowStartTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i64>,
}

/// Software update options for the domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainSoftwareUpdateOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoSoftwareUpdateEnabled")]
    pub auto_software_update_enabled: Option<bool>,
}

/// A tag (key-value pair) for an Amazon OpenSearch Service resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainTags {
    /// A string between 1 to 128 characters that specifies the key for a tag. Tag
    /// keys must be unique for the domain to which they're attached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// A string between 0 to 256 characters that specifies the value for a tag.
    /// Tag values can be null and don't have to be unique in a tag set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Container for the values required to configure VPC access domains. If you
/// don't specify these values, OpenSearch Service creates the domain with a
/// public endpoint. For more information, see Launching your Amazon OpenSearch
/// Service domains using a VPC (https://docs.aws.amazon.com/opensearch-service/latest/developerguide/vpc.html).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainVpcOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityGroupIDs")]
    pub security_group_i_ds: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subnetIDs")]
    pub subnet_i_ds: Option<Vec<String>>,
}

/// DomainStatus defines the observed state of Domain
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<DomainStatusAckResourceMetadata>,
    /// Information about a configuration change happening on the domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "changeProgressDetails")]
    pub change_progress_details: Option<DomainStatusChangeProgressDetails>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// Creation status of an OpenSearch Service domain. True if domain creation
    /// is complete. False if domain creation is still in progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    /// Deletion status of an OpenSearch Service domain. True if domain deletion
    /// is complete. False if domain deletion is still in progress. Once deletion
    /// is complete, the status of the domain is no longer returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// The dual stack hosted zone ID for the domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainEndpointV2HostedZoneID")]
    pub domain_endpoint_v2_hosted_zone_id: Option<String>,
    /// Unique identifier for the domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainID")]
    pub domain_id: Option<String>,
    /// The status of any changes that are currently in progress for the domain.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domainProcessingStatus")]
    pub domain_processing_status: Option<String>,
    /// Domain-specific endpoint used to submit index, search, and data upload requests
    /// to the domain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// If IPAddressType to set to dualstack, a version 2 domain endpoint is provisioned.
    /// This endpoint functions like a normal endpoint, except that it works with
    /// both IPv4 and IPv6 IP addresses. Normal endpoints work only with IPv4 IP
    /// addresses.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointV2")]
    pub endpoint_v2: Option<String>,
    /// The key-value pair that exists if the OpenSearch Service domain uses VPC
    /// endpoints. For example:
    /// 
    ///    * IPv4 IP addresses - 'vpc','vpc-endpoint-h2dsd34efgyghrtguk5gt6j2foh4.us-east-1.es.amazonaws.com'
    /// 
    ///    * Dual stack IP addresses - 'vpcv2':'vpc-endpoint-h2dsd34efgyghrtguk5gt6j2foh4.aos.us-east-1.on.aws'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<BTreeMap<String, String>>,
    /// Information about the domain properties that are currently being modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modifyingProperties")]
    pub modifying_properties: Option<Vec<DomainStatusModifyingProperties>>,
    /// The status of the domain configuration. True if OpenSearch Service is processing
    /// configuration changes. False if the configuration is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    /// The current status of the domain's service software.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSoftwareOptions")]
    pub service_software_options: Option<DomainStatusServiceSoftwareOptions>,
    /// DEPRECATED. Container for parameters required to configure automated snapshots
    /// of domain indexes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "snapshotOptions")]
    pub snapshot_options: Option<DomainStatusSnapshotOptions>,
    /// The status of a domain version upgrade to a new version of OpenSearch or
    /// Elasticsearch. True if OpenSearch Service is in the process of a version
    /// upgrade. False if the configuration is active.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeProcessing")]
    pub upgrade_processing: Option<bool>,
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

/// Information about a configuration change happening on the domain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatusChangeProgressDetails {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "changeID")]
    pub change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configChangeStatus")]
    pub config_change_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initiatedBy")]
    pub initiated_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdatedTime")]
    pub last_updated_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// Information about the domain properties that are currently being modified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatusModifyingProperties {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeValue")]
    pub active_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingValue")]
    pub pending_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueType")]
    pub value_type: Option<String>,
}

/// The current status of the domain's service software.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatusServiceSoftwareOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "automatedUpdateDate")]
    pub automated_update_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentVersion")]
    pub current_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newVersion")]
    pub new_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "optionalDeployment")]
    pub optional_deployment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateAvailable")]
    pub update_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStatus")]
    pub update_status: Option<String>,
}

/// DEPRECATED. Container for parameters required to configure automated snapshots
/// of domain indexes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DomainStatusSnapshotOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "automatedSnapshotStartHour")]
    pub automated_snapshot_start_hour: Option<i64>,
}

