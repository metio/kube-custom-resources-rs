// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/cloudfront-controller/cloudfront.services.k8s.aws/v1alpha1/originrequestpolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// OriginRequestPolicySpec defines the desired state of OriginRequestPolicy.
/// 
/// An origin request policy.
/// 
/// When it's attached to a cache behavior, the origin request policy determines
/// the values that CloudFront includes in requests that it sends to the origin.
/// Each request that CloudFront sends to the origin includes the following:
/// 
///    * The request body and the URL path (without the domain name) from the
///    viewer request.
/// 
///    * The headers that CloudFront automatically includes in every origin request,
///    including Host, User-Agent, and X-Amz-Cf-Id.
/// 
///    * All HTTP headers, cookies, and URL query strings that are specified
///    in the cache policy or the origin request policy. These can include items
///    from the viewer request and, in the case of headers, additional ones that
///    are added by CloudFront.
/// 
/// CloudFront sends a request when it can't find an object in its cache that
/// matches the request. If you want to send values to the origin and also include
/// them in the cache key, use CachePolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cloudfront.services.k8s.aws", version = "v1alpha1", kind = "OriginRequestPolicy", plural = "originrequestpolicies")]
#[kube(namespaced)]
#[kube(status = "OriginRequestPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct OriginRequestPolicySpec {
    /// An origin request policy configuration.
    #[serde(rename = "originRequestPolicyConfig")]
    pub origin_request_policy_config: OriginRequestPolicyOriginRequestPolicyConfig,
}

/// An origin request policy configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// An object that determines whether any cookies in viewer requests (and if
    /// so, which cookies) are included in requests that CloudFront sends to the
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cookiesConfig")]
    pub cookies_config: Option<OriginRequestPolicyOriginRequestPolicyConfigCookiesConfig>,
    /// An object that determines whether any HTTP headers (and if so, which headers)
    /// are included in requests that CloudFront sends to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headersConfig")]
    pub headers_config: Option<OriginRequestPolicyOriginRequestPolicyConfigHeadersConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An object that determines whether any URL query strings in viewer requests
    /// (and if so, which query strings) are included in requests that CloudFront
    /// sends to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStringsConfig")]
    pub query_strings_config: Option<OriginRequestPolicyOriginRequestPolicyConfigQueryStringsConfig>,
}

/// An object that determines whether any cookies in viewer requests (and if
/// so, which cookies) are included in requests that CloudFront sends to the
/// origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigCookiesConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cookieBehavior")]
    pub cookie_behavior: Option<String>,
    /// Contains a list of cookie names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookies: Option<OriginRequestPolicyOriginRequestPolicyConfigCookiesConfigCookies>,
}

/// Contains a list of cookie names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigCookiesConfigCookies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// An object that determines whether any HTTP headers (and if so, which headers)
/// are included in requests that CloudFront sends to the origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigHeadersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerBehavior")]
    pub header_behavior: Option<String>,
    /// Contains a list of HTTP header names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<OriginRequestPolicyOriginRequestPolicyConfigHeadersConfigHeaders>,
}

/// Contains a list of HTTP header names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigHeadersConfigHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// An object that determines whether any URL query strings in viewer requests
/// (and if so, which query strings) are included in requests that CloudFront
/// sends to the origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigQueryStringsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStringBehavior")]
    pub query_string_behavior: Option<String>,
    /// Contains a list of query string names.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStrings")]
    pub query_strings: Option<OriginRequestPolicyOriginRequestPolicyConfigQueryStringsConfigQueryStrings>,
}

/// Contains a list of query string names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyOriginRequestPolicyConfigQueryStringsConfigQueryStrings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// OriginRequestPolicyStatus defines the observed state of OriginRequestPolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<OriginRequestPolicyStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eTag")]
    pub e_tag: Option<String>,
    /// The unique identifier for the origin request policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time when the origin request policy was last modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OriginRequestPolicyStatusAckResourceMetadata {
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

