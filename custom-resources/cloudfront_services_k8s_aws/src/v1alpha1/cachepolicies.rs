// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/cloudfront-controller/cloudfront.services.k8s.aws/v1alpha1/cachepolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// CachePolicySpec defines the desired state of CachePolicy.
/// 
/// A cache policy.
/// 
/// When it's attached to a cache behavior, the cache policy determines the following:
/// 
///    * The values that CloudFront includes in the cache key. These values can
///    include HTTP headers, cookies, and URL query strings. CloudFront uses
///    the cache key to find an object in its cache that it can return to the
///    viewer.
/// 
///    * The default, minimum, and maximum time to live (TTL) values that you
///    want objects to stay in the CloudFront cache.
/// 
/// The headers, cookies, and query strings that are included in the cache key
/// are also included in requests that CloudFront sends to the origin. CloudFront
/// sends a request when it can't find a valid object in its cache that matches
/// the request's cache key. If you want to send values to the origin but not
/// include them in the cache key, use OriginRequestPolicy.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cloudfront.services.k8s.aws", version = "v1alpha1", kind = "CachePolicy", plural = "cachepolicies")]
#[kube(namespaced)]
#[kube(status = "CachePolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CachePolicySpec {
    /// A cache policy configuration.
    #[serde(rename = "cachePolicyConfig")]
    pub cache_policy_config: CachePolicyCachePolicyConfig,
}

/// A cache policy configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultTTL")]
    pub default_ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minTTL")]
    pub min_ttl: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// This object determines the values that CloudFront includes in the cache key.
    /// These values can include HTTP headers, cookies, and URL query strings. CloudFront
    /// uses the cache key to find an object in its cache that it can return to the
    /// viewer.
    /// 
    /// The headers, cookies, and query strings that are included in the cache key
    /// are also included in requests that CloudFront sends to the origin. CloudFront
    /// sends a request when it can't find an object in its cache that matches the
    /// request's cache key. If you want to send values to the origin but not include
    /// them in the cache key, use OriginRequestPolicy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parametersInCacheKeyAndForwardedToOrigin")]
    pub parameters_in_cache_key_and_forwarded_to_origin: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOrigin>,
}

/// This object determines the values that CloudFront includes in the cache key.
/// These values can include HTTP headers, cookies, and URL query strings. CloudFront
/// uses the cache key to find an object in its cache that it can return to the
/// viewer.
/// 
/// The headers, cookies, and query strings that are included in the cache key
/// are also included in requests that CloudFront sends to the origin. CloudFront
/// sends a request when it can't find an object in its cache that matches the
/// request's cache key. If you want to send values to the origin but not include
/// them in the cache key, use OriginRequestPolicy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOrigin {
    /// An object that determines whether any cookies in viewer requests (and if
    /// so, which cookies) are included in the cache key and in requests that CloudFront
    /// sends to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cookiesConfig")]
    pub cookies_config: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableAcceptEncodingBrotli")]
    pub enable_accept_encoding_brotli: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableAcceptEncodingGzip")]
    pub enable_accept_encoding_gzip: Option<bool>,
    /// An object that determines whether any HTTP headers (and if so, which headers)
    /// are included in the cache key and in requests that CloudFront sends to the
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headersConfig")]
    pub headers_config: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// An object that determines whether any URL query strings in viewer requests
    /// (and if so, which query strings) are included in the cache key and in requests
    /// that CloudFront sends to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStringsConfig")]
    pub query_strings_config: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}

/// An object that determines whether any cookies in viewer requests (and if
/// so, which cookies) are included in the cache key and in requests that CloudFront
/// sends to the origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginCookiesConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cookieBehavior")]
    pub cookie_behavior: Option<String>,
    /// Contains a list of cookie names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookies: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies>,
}

/// Contains a list of cookie names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// An object that determines whether any HTTP headers (and if so, which headers)
/// are included in the cache key and in requests that CloudFront sends to the
/// origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginHeadersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerBehavior")]
    pub header_behavior: Option<String>,
    /// Contains a list of HTTP header names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders>,
}

/// Contains a list of HTTP header names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// An object that determines whether any URL query strings in viewer requests
/// (and if so, which query strings) are included in the cache key and in requests
/// that CloudFront sends to the origin.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginQueryStringsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStringBehavior")]
    pub query_string_behavior: Option<String>,
    /// Contains a list of query string names.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryStrings")]
    pub query_strings: Option<CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings>,
}

/// Contains a list of query string names.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyCachePolicyConfigParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// CachePolicyStatus defines the observed state of CachePolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<CachePolicyStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The current version of the cache policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eTag")]
    pub e_tag: Option<String>,
    /// The unique identifier for the cache policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time when the cache policy was last modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CachePolicyStatusAckResourceMetadata {
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

