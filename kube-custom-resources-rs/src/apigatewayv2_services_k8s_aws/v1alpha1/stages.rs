// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/aws-controllers-k8s/apigatewayv2-controller/apigatewayv2.services.k8s.aws/v1alpha1/stages.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// StageSpec defines the desired state of Stage.
/// 
/// Represents an API stage.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apigatewayv2.services.k8s.aws", version = "v1alpha1", kind = "Stage", plural = "stages")]
#[kube(namespaced)]
#[kube(status = "StageStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct StageSpec {
    /// Settings for logging access in this stage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessLogSettings")]
    pub access_log_settings: Option<StageAccessLogSettings>,
    /// The API identifier.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiID")]
    pub api_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiRef")]
    pub api_ref: Option<StageApiRef>,
    /// Specifies whether updates to an API automatically trigger a new deployment.
    /// The default value is false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoDeploy")]
    pub auto_deploy: Option<bool>,
    /// The identifier of a client certificate for a Stage. Supported only for WebSocket
    /// APIs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertificateID")]
    pub client_certificate_id: Option<String>,
    /// The default route settings for the stage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultRouteSettings")]
    pub default_route_settings: Option<StageDefaultRouteSettings>,
    /// The deployment identifier of the API stage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentID")]
    pub deployment_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentRef")]
    pub deployment_ref: Option<StageDeploymentRef>,
    /// The description for the API stage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Route settings for the stage, by routeKey.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeSettings")]
    pub route_settings: Option<BTreeMap<String, StageRouteSettings>>,
    /// The name of the stage.
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// A map that defines the stage variables for a Stage. Variable names can have
    /// alphanumeric and underscore characters, and the values must match [A-Za-z0-9-._~:/?#&=,]+.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stageVariables")]
    pub stage_variables: Option<BTreeMap<String, String>>,
    /// The collection of tags. Each tag element is associated with a given resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// Settings for logging access in this stage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageAccessLogSettings {
    /// Represents an Amazon Resource Name (ARN).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationARN")]
    pub destination_arn: Option<String>,
    /// A string with a length between [1-1024].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageApiRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<StageApiRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageApiRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The default route settings for the stage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageDefaultRouteSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detailedMetricsEnabled")]
    pub detailed_metrics_enabled: Option<bool>,
    /// The logging level.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loggingLevel")]
    pub logging_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttlingBurstLimit")]
    pub throttling_burst_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageDeploymentRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<StageDeploymentRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageDeploymentRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Route settings for the stage, by routeKey.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageRouteSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataTraceEnabled")]
    pub data_trace_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detailedMetricsEnabled")]
    pub detailed_metrics_enabled: Option<bool>,
    /// The logging level.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loggingLevel")]
    pub logging_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttlingBurstLimit")]
    pub throttling_burst_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttlingRateLimit")]
    pub throttling_rate_limit: Option<f64>,
}

/// StageStatus defines the observed state of Stage
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<StageStatusAckResourceMetadata>,
    /// Specifies whether a stage is managed by API Gateway. If you created an API
    /// using quick create, the $default stage is managed by API Gateway. You can't
    /// modify the $default stage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGatewayManaged")]
    pub api_gateway_managed: Option<bool>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The timestamp when the stage was created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdDate")]
    pub created_date: Option<String>,
    /// Describes the status of the last deployment of a stage. Supported only for
    /// stages with autoDeploy enabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastDeploymentStatusMessage")]
    pub last_deployment_status_message: Option<String>,
    /// The timestamp when the stage was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdatedDate")]
    pub last_updated_date: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StageStatusAckResourceMetadata {
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

