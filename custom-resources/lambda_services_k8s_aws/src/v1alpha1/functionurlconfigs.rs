// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/lambda-controller/lambda.services.k8s.aws/v1alpha1/functionurlconfigs.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// FunctionUrlConfigSpec defines the desired state of FunctionUrlConfig.
/// 
/// Details about a Lambda function URL.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "lambda.services.k8s.aws", version = "v1alpha1", kind = "FunctionURLConfig", plural = "functionurlconfigs")]
#[kube(namespaced)]
#[kube(status = "FunctionURLConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FunctionURLConfigSpec {
    /// The type of authentication that your function URL uses. Set to AWS_IAM if
    /// you want to restrict access to authenticated IAM users only. Set to NONE
    /// if you want to bypass IAM authentication to create a public endpoint. For
    /// more information, see Security and auth model for Lambda function URLs (https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html).
    #[serde(rename = "authType")]
    pub auth_type: String,
    /// The cross-origin resource sharing (CORS) (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
    /// settings for your function URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<FunctionURLConfigCors>,
    /// The name of the Lambda function.
    /// 
    /// Name formats
    /// 
    ///    * Function name – my-function.
    /// 
    ///    * Function ARN – arn:aws:lambda:us-west-2:123456789012:function:my-function.
    /// 
    ///    * Partial ARN – 123456789012:function:my-function.
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
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionRef")]
    pub function_ref: Option<FunctionURLConfigFunctionRef>,
    /// The alias name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// The cross-origin resource sharing (CORS) (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
/// settings for your function URL.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FunctionURLConfigCors {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCredentials")]
    pub allow_credentials: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowHeaders")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMethods")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigins")]
    pub allow_origins: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposeHeaders")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i64>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FunctionURLConfigFunctionRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FunctionURLConfigFunctionRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FunctionURLConfigFunctionRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// FunctionURLConfigStatus defines the observed state of FunctionURLConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FunctionURLConfigStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<FunctionURLConfigStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// When the function URL was created, in ISO-8601 format (https://www.w3.org/TR/NOTE-datetime)
    /// (YYYY-MM-DDThh:mm:ss.sTZD).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<String>,
    /// The Amazon Resource Name (ARN) of your function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionARN")]
    pub function_arn: Option<String>,
    /// The HTTP URL endpoint for your function.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "functionURL")]
    pub function_url: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FunctionURLConfigStatusAckResourceMetadata {
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

