// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/cloudfront-controller/cloudfront.services.k8s.aws/v1alpha1/responseheaderspolicies.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// ResponseHeadersPolicySpec defines the desired state of ResponseHeadersPolicy.
/// 
/// A response headers policy.
/// 
/// A response headers policy contains information about a set of HTTP response
/// headers.
/// 
/// After you create a response headers policy, you can use its ID to attach
/// it to one or more cache behaviors in a CloudFront distribution. When it's
/// attached to a cache behavior, the response headers policy affects the HTTP
/// headers that CloudFront includes in HTTP responses to requests that match
/// the cache behavior. CloudFront adds or removes response headers according
/// to the configuration of the response headers policy.
/// 
/// For more information, see Adding or removing HTTP headers in CloudFront responses
/// (https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/modifying-response-headers.html)
/// in the Amazon CloudFront Developer Guide.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cloudfront.services.k8s.aws", version = "v1alpha1", kind = "ResponseHeadersPolicy", plural = "responseheaderspolicies")]
#[kube(namespaced)]
#[kube(status = "ResponseHeadersPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ResponseHeadersPolicySpec {
    /// Contains metadata about the response headers policy, and a set of configurations
    /// that specify the HTTP headers.
    #[serde(rename = "responseHeadersPolicyConfig")]
    pub response_headers_policy_config: ResponseHeadersPolicyResponseHeadersPolicyConfig,
}

/// Contains metadata about the response headers policy, and a set of configurations
/// that specify the HTTP headers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A configuration for a set of HTTP response headers that are used for cross-origin
    /// resource sharing (CORS). CloudFront adds these headers to HTTP responses
    /// that it sends for CORS requests that match a cache behavior associated with
    /// this response headers policy.
    /// 
    /// For more information about CORS, see Cross-Origin Resource Sharing (CORS)
    /// (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "corsConfig")]
    pub cors_config: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfig>,
    /// A list of HTTP response header names and their values. CloudFront includes
    /// these headers in HTTP responses that it sends for requests that match a cache
    /// behavior that's associated with this response headers policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customHeadersConfig")]
    pub custom_headers_config: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCustomHeadersConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of HTTP header names that CloudFront removes from HTTP responses to
    /// requests that match the cache behavior that this response headers policy
    /// is attached to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "removeHeadersConfig")]
    pub remove_headers_config: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigRemoveHeadersConfig>,
    /// A configuration for a set of security-related HTTP response headers. CloudFront
    /// adds these headers to HTTP responses that it sends for requests that match
    /// a cache behavior associated with this response headers policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityHeadersConfig")]
    pub security_headers_config: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfig>,
    /// A configuration for enabling the Server-Timing header in HTTP responses sent
    /// from CloudFront. CloudFront adds this header to HTTP responses that it sends
    /// in response to requests that match a cache behavior that's associated with
    /// this response headers policy.
    /// 
    /// You can use the Server-Timing header to view metrics that can help you gain
    /// insights about the behavior and performance of CloudFront. For example, you
    /// can see which cache layer served a cache hit, or the first byte latency from
    /// the origin when there was a cache miss. You can use the metrics in the Server-Timing
    /// header to troubleshoot issues or test the efficiency of your CloudFront configuration.
    /// For more information, see Server-Timing header (https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/understanding-response-headers-policies.html#server-timing-header)
    /// in the Amazon CloudFront Developer Guide.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverTimingHeadersConfig")]
    pub server_timing_headers_config: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigServerTimingHeadersConfig>,
}

/// A configuration for a set of HTTP response headers that are used for cross-origin
/// resource sharing (CORS). CloudFront adds these headers to HTTP responses
/// that it sends for CORS requests that match a cache behavior associated with
/// this response headers policy.
/// 
/// For more information about CORS, see Cross-Origin Resource Sharing (CORS)
/// (https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlAllowCredentials")]
    pub access_control_allow_credentials: Option<bool>,
    /// A list of HTTP header names that CloudFront includes as values for the Access-Control-Allow-Headers
    /// HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Headers HTTP response
    /// header, see Access-Control-Allow-Headers (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlAllowHeaders")]
    pub access_control_allow_headers: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowHeaders>,
    /// A list of HTTP methods that CloudFront includes as values for the Access-Control-Allow-Methods
    /// HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Methods HTTP response
    /// header, see Access-Control-Allow-Methods (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlAllowMethods")]
    pub access_control_allow_methods: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowMethods>,
    /// A list of origins (domain names) that CloudFront can use as the value for
    /// the Access-Control-Allow-Origin HTTP response header.
    /// 
    /// For more information about the Access-Control-Allow-Origin HTTP response
    /// header, see Access-Control-Allow-Origin (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlAllowOrigins")]
    pub access_control_allow_origins: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowOrigins>,
    /// A list of HTTP headers that CloudFront includes as values for the Access-Control-Expose-Headers
    /// HTTP response header.
    /// 
    /// For more information about the Access-Control-Expose-Headers HTTP response
    /// header, see Access-Control-Expose-Headers (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlExposeHeaders")]
    pub access_control_expose_headers: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlExposeHeaders>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlMaxAgeSec")]
    pub access_control_max_age_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "originOverride")]
    pub origin_override: Option<bool>,
}

/// A list of HTTP header names that CloudFront includes as values for the Access-Control-Allow-Headers
/// HTTP response header.
/// 
/// For more information about the Access-Control-Allow-Headers HTTP response
/// header, see Access-Control-Allow-Headers (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// A list of HTTP methods that CloudFront includes as values for the Access-Control-Allow-Methods
/// HTTP response header.
/// 
/// For more information about the Access-Control-Allow-Methods HTTP response
/// header, see Access-Control-Allow-Methods (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Methods)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowMethods {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// A list of origins (domain names) that CloudFront can use as the value for
/// the Access-Control-Allow-Origin HTTP response header.
/// 
/// For more information about the Access-Control-Allow-Origin HTTP response
/// header, see Access-Control-Allow-Origin (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlAllowOrigins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// A list of HTTP headers that CloudFront includes as values for the Access-Control-Expose-Headers
/// HTTP response header.
/// 
/// For more information about the Access-Control-Expose-Headers HTTP response
/// header, see Access-Control-Expose-Headers (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Expose-Headers)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCorsConfigAccessControlExposeHeaders {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// A list of HTTP response header names and their values. CloudFront includes
/// these headers in HTTP responses that it sends for requests that match a cache
/// behavior that's associated with this response headers policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCustomHeadersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ResponseHeadersPolicyResponseHeadersPolicyConfigCustomHeadersConfigItems>>,
}

/// An HTTP response header name and its value. CloudFront includes this header
/// in HTTP responses that it sends for requests that match a cache behavior
/// that's associated with this response headers policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigCustomHeadersConfigItems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// A list of HTTP header names that CloudFront removes from HTTP responses to
/// requests that match the cache behavior that this response headers policy
/// is attached to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigRemoveHeadersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ResponseHeadersPolicyResponseHeadersPolicyConfigRemoveHeadersConfigItems>>,
}

/// The name of an HTTP header that CloudFront removes from HTTP responses to
/// requests that match the cache behavior that this response headers policy
/// is attached to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigRemoveHeadersConfigItems {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
}

/// A configuration for a set of security-related HTTP response headers. CloudFront
/// adds these headers to HTTP responses that it sends for requests that match
/// a cache behavior associated with this response headers policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfig {
    /// The policy directives and their values that CloudFront includes as values
    /// for the Content-Security-Policy HTTP response header.
    /// 
    /// For more information about the Content-Security-Policy HTTP response header,
    /// see Content-Security-Policy (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentSecurityPolicy")]
    pub content_security_policy: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigContentSecurityPolicy>,
    /// Determines whether CloudFront includes the X-Content-Type-Options HTTP response
    /// header with its value set to nosniff.
    /// 
    /// For more information about the X-Content-Type-Options HTTP response header,
    /// see X-Content-Type-Options (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentTypeOptions")]
    pub content_type_options: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigContentTypeOptions>,
    /// Determines whether CloudFront includes the X-Frame-Options HTTP response
    /// header and the header's value.
    /// 
    /// For more information about the X-Frame-Options HTTP response header, see
    /// X-Frame-Options (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frameOptions")]
    pub frame_options: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigFrameOptions>,
    /// Determines whether CloudFront includes the Referrer-Policy HTTP response
    /// header and the header's value.
    /// 
    /// For more information about the Referrer-Policy HTTP response header, see
    /// Referrer-Policy (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "referrerPolicy")]
    pub referrer_policy: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigReferrerPolicy>,
    /// Determines whether CloudFront includes the Strict-Transport-Security HTTP
    /// response header and the header's value.
    /// 
    /// For more information about the Strict-Transport-Security HTTP response header,
    /// see Strict-Transport-Security (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "strictTransportSecurity")]
    pub strict_transport_security: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigStrictTransportSecurity>,
    /// Determines whether CloudFront includes the X-XSS-Protection HTTP response
    /// header and the header's value.
    /// 
    /// For more information about the X-XSS-Protection HTTP response header, see
    /// X-XSS-Protection (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-XSS-Protection)
    /// in the MDN Web Docs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xssProtection")]
    pub xss_protection: Option<ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigXssProtection>,
}

/// The policy directives and their values that CloudFront includes as values
/// for the Content-Security-Policy HTTP response header.
/// 
/// For more information about the Content-Security-Policy HTTP response header,
/// see Content-Security-Policy (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigContentSecurityPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentSecurityPolicy")]
    pub content_security_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
}

/// Determines whether CloudFront includes the X-Content-Type-Options HTTP response
/// header with its value set to nosniff.
/// 
/// For more information about the X-Content-Type-Options HTTP response header,
/// see X-Content-Type-Options (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigContentTypeOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
}

/// Determines whether CloudFront includes the X-Frame-Options HTTP response
/// header and the header's value.
/// 
/// For more information about the X-Frame-Options HTTP response header, see
/// X-Frame-Options (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigFrameOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "frameOption")]
    pub frame_option: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
}

/// Determines whether CloudFront includes the Referrer-Policy HTTP response
/// header and the header's value.
/// 
/// For more information about the Referrer-Policy HTTP response header, see
/// Referrer-Policy (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigReferrerPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "referrerPolicy")]
    pub referrer_policy: Option<String>,
}

/// Determines whether CloudFront includes the Strict-Transport-Security HTTP
/// response header and the header's value.
/// 
/// For more information about the Strict-Transport-Security HTTP response header,
/// see Strict-Transport-Security (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigStrictTransportSecurity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessControlMaxAgeSec")]
    pub access_control_max_age_sec: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "includeSubdomains")]
    pub include_subdomains: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preload: Option<bool>,
}

/// Determines whether CloudFront includes the X-XSS-Protection HTTP response
/// header and the header's value.
/// 
/// For more information about the X-XSS-Protection HTTP response header, see
/// X-XSS-Protection (https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-XSS-Protection)
/// in the MDN Web Docs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigSecurityHeadersConfigXssProtection {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modeBlock")]
    pub mode_block: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "override")]
    pub r#override: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportURI")]
    pub report_uri: Option<String>,
}

/// A configuration for enabling the Server-Timing header in HTTP responses sent
/// from CloudFront. CloudFront adds this header to HTTP responses that it sends
/// in response to requests that match a cache behavior that's associated with
/// this response headers policy.
/// 
/// You can use the Server-Timing header to view metrics that can help you gain
/// insights about the behavior and performance of CloudFront. For example, you
/// can see which cache layer served a cache hit, or the first byte latency from
/// the origin when there was a cache miss. You can use the metrics in the Server-Timing
/// header to troubleshoot issues or test the efficiency of your CloudFront configuration.
/// For more information, see Server-Timing header (https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/understanding-response-headers-policies.html#server-timing-header)
/// in the Amazon CloudFront Developer Guide.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyResponseHeadersPolicyConfigServerTimingHeadersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "samplingRate")]
    pub sampling_rate: Option<f64>,
}

/// ResponseHeadersPolicyStatus defines the observed state of ResponseHeadersPolicy
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<ResponseHeadersPolicyStatusAckResourceMetadata>,
    /// All CRS managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "eTag")]
    pub e_tag: Option<String>,
    /// The identifier for the response headers policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time when the response headers policy was last modified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ResponseHeadersPolicyStatusAckResourceMetadata {
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

