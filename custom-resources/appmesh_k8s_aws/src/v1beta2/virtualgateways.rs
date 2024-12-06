// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/aws-app-mesh-controller-for-k8/appmesh.k8s.aws/v1beta2/virtualgateways.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VirtualGatewaySpec defines the desired state of VirtualGateway refers to https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_gateways.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "appmesh.k8s.aws", version = "v1beta2", kind = "VirtualGateway", plural = "virtualgateways")]
#[kube(namespaced)]
#[kube(status = "VirtualGatewayStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VirtualGatewaySpec {
    /// AWSName is the AppMesh VirtualGateway object's name. If unspecified or empty, it defaults to be "${name}_${namespace}" of k8s VirtualGateway
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsName")]
    pub aws_name: Option<String>,
    /// A reference to an object that represents the defaults for backend GatewayRoutes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendDefaults")]
    pub backend_defaults: Option<VirtualGatewayBackendDefaults>,
    /// GatewayRouteSelector selects GatewayRoutes using labels to designate GatewayRoute membership. If not specified it selects all GatewayRoutes in that namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gatewayRouteSelector")]
    pub gateway_route_selector: Option<VirtualGatewayGatewayRouteSelector>,
    /// The listener that the virtual gateway is expected to receive inbound traffic from
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<VirtualGatewayListeners>>,
    /// The inbound and outbound access logging information for the virtual gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<VirtualGatewayLogging>,
    /// A reference to k8s Mesh CR that this VirtualGateway belongs to. The admission controller populates it using Meshes's selector, and prevents users from setting this field. 
    ///  Populated by the system. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "meshRef")]
    pub mesh_ref: Option<VirtualGatewayMeshRef>,
    /// NamespaceSelector selects Namespaces using labels to designate GatewayRoute membership. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<VirtualGatewayNamespaceSelector>,
    /// PodSelector selects Pods using labels to designate VirtualGateway membership. This field follows standard label selector semantics: 	if present but empty, it selects all pods within namespace. 	if absent, it selects no pod.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podSelector")]
    pub pod_selector: Option<VirtualGatewayPodSelector>,
}

/// A reference to an object that represents the defaults for backend GatewayRoutes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaults {
    /// A reference to an object that represents a client policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientPolicy")]
    pub client_policy: Option<VirtualGatewayBackendDefaultsClientPolicy>,
}

/// A reference to an object that represents a client policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicy {
    /// A reference to an object that represents a Transport Layer Security (TLS) client policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayBackendDefaultsClientPolicyTls>,
}

/// A reference to an object that represents a Transport Layer Security (TLS) client policy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTls {
    /// A reference to an object that represents TLS certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<VirtualGatewayBackendDefaultsClientPolicyTlsCertificate>,
    /// Whether the policy is enforced. If unspecified, default settings from AWS API will be applied. Refer to AWS Docs for default settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce: Option<bool>,
    /// The range of ports that the policy is enforced for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i64>>,
    /// A reference to an object that represents a TLS validation context.
    pub validation: VirtualGatewayBackendDefaultsClientPolicyTlsValidation,
}

/// A reference to an object that represents TLS certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsCertificate {
    /// An object that represents a TLS cert via a local file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayBackendDefaultsClientPolicyTlsCertificateFile>,
    /// An object that represents a TLS cert via SDS entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayBackendDefaultsClientPolicyTlsCertificateSds>,
}

/// An object that represents a TLS cert via a local file
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsCertificateFile {
    /// The certificate chain for the certificate.
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
    /// The private key for a certificate stored on the file system of the virtual Gateway.
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// An object that represents a TLS cert via SDS entry
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsCertificateSds {
    /// The certificate trust chain for a certificate issued via SDS cluster
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// A reference to an object that represents a TLS validation context.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidation {
    /// Possible alternative names to consider
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAlternativeNames")]
    pub subject_alternative_names: Option<VirtualGatewayBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames>,
    /// A reference to an object that represents a TLS validation context trust
    pub trust: VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrust,
}

/// Possible alternative names to consider
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames {
    /// Match is a required field
    #[serde(rename = "match")]
    pub r#match: VirtualGatewayBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch,
}

/// Match is a required field
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNamesMatch {
    /// Exact is a required field
    pub exact: Vec<String>,
}

/// A reference to an object that represents a TLS validation context trust
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrust {
    /// A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustAcm>,
    /// An object that represents a TLS validation context trust for a local file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustFile>,
    /// An object that represents a TLS validation context trust for a SDS certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustSds>,
}

/// A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustAcm {
    /// One or more ACM Amazon Resource Name (ARN)s.
    #[serde(rename = "certificateAuthorityARNs")]
    pub certificate_authority_ar_ns: Vec<String>,
}

/// An object that represents a TLS validation context trust for a local file.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustFile {
    /// The certificate trust chain for a certificate stored on the file system of the virtual Gateway.
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
}

/// An object that represents a TLS validation context trust for a SDS certificate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayBackendDefaultsClientPolicyTlsValidationTrustSds {
    /// The certificate trust chain for a certificate issued via SDS.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// GatewayRouteSelector selects GatewayRoutes using labels to designate GatewayRoute membership. If not specified it selects all GatewayRoutes in that namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayGatewayRouteSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VirtualGatewayGatewayRouteSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayGatewayRouteSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// VirtualGatewayListener refers to https://docs.aws.amazon.com/app-mesh/latest/userguide/virtual_gateways.html
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VirtualGatewayListeners {
    /// The connection pool settings for the listener
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionPool")]
    pub connection_pool: Option<VirtualGatewayListenersConnectionPool>,
    /// The health check information for the listener.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<VirtualGatewayListenersHealthCheck>,
    /// The port mapping information for the listener.
    #[serde(rename = "portMapping")]
    pub port_mapping: VirtualGatewayListenersPortMapping,
    /// A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<VirtualGatewayListenersTls>,
}

/// The connection pool settings for the listener
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersConnectionPool {
    /// Specifies grpc connection pool settings for the virtual gateway listener
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<VirtualGatewayListenersConnectionPoolGrpc>,
    /// Specifies http connection pool settings for the virtual gateway listener
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<VirtualGatewayListenersConnectionPoolHttp>,
    /// Specifies http2 connection pool settings for the virtual gateway listener
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http2: Option<VirtualGatewayListenersConnectionPoolHttp2>,
}

/// Specifies grpc connection pool settings for the virtual gateway listener
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersConnectionPoolGrpc {
    /// Represents the maximum number of inflight requests that an envoy can concurrently support across all the hosts in the upstream cluster
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// Specifies http connection pool settings for the virtual gateway listener
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersConnectionPoolHttp {
    /// Represents the maximum number of outbound TCP connections the envoy can establish concurrently with all the hosts in the upstream cluster.
    #[serde(rename = "maxConnections")]
    pub max_connections: i64,
    /// Represents the number of overflowing requests after max_connections that an envoy will queue to an upstream cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxPendingRequests")]
    pub max_pending_requests: Option<i64>,
}

/// Specifies http2 connection pool settings for the virtual gateway listener
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersConnectionPoolHttp2 {
    /// Represents the maximum number of inflight requests that an envoy can concurrently support across all the hosts in the upstream cluster
    #[serde(rename = "maxRequests")]
    pub max_requests: i64,
}

/// The health check information for the listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VirtualGatewayListenersHealthCheck {
    /// The number of consecutive successful health checks that must occur before declaring listener healthy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthyThreshold")]
    pub healthy_threshold: Option<i64>,
    /// The time period in milliseconds between each health check execution.
    #[serde(rename = "intervalMillis")]
    pub interval_millis: i64,
    /// The destination path for the health check request. This value is only used if the specified protocol is http or http2. For any other protocol, this value is ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The destination port for the health check request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// The protocol for the health check request
    pub protocol: VirtualGatewayListenersHealthCheckProtocol,
    /// The amount of time to wait when receiving a response from the health check, in milliseconds.
    #[serde(rename = "timeoutMillis")]
    pub timeout_millis: i64,
    /// The number of consecutive failed health checks that must occur before declaring a virtual Gateway unhealthy.
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i64,
}

/// The health check information for the listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VirtualGatewayListenersHealthCheckProtocol {
    #[serde(rename = "grpc")]
    Grpc,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "http2")]
    Http2,
}

/// The port mapping information for the listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VirtualGatewayListenersPortMapping {
    /// The port used for the port mapping.
    pub port: i64,
    /// The protocol used for the port mapping.
    pub protocol: VirtualGatewayListenersPortMappingProtocol,
}

/// The port mapping information for the listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VirtualGatewayListenersPortMappingProtocol {
    #[serde(rename = "grpc")]
    Grpc,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "http2")]
    Http2,
}

/// A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VirtualGatewayListenersTls {
    /// A reference to an object that represents a listener's TLS certificate.
    pub certificate: VirtualGatewayListenersTlsCertificate,
    /// ListenerTLS mode
    pub mode: VirtualGatewayListenersTlsMode,
    /// A reference to an object that represents Validation context
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<VirtualGatewayListenersTlsValidation>,
}

/// A reference to an object that represents a listener's TLS certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsCertificate {
    /// A reference to an object that represents an AWS Certificate Manager (ACM) certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayListenersTlsCertificateAcm>,
    /// A reference to an object that represents a local file certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayListenersTlsCertificateFile>,
    /// A reference to an object that represents an SDS issued certificate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayListenersTlsCertificateSds>,
}

/// A reference to an object that represents an AWS Certificate Manager (ACM) certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsCertificateAcm {
    /// The Amazon Resource Name (ARN) for the certificate.
    #[serde(rename = "certificateARN")]
    pub certificate_arn: String,
}

/// A reference to an object that represents a local file certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsCertificateFile {
    /// The certificate chain for the certificate.
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
    /// The private key for a certificate stored on the file system of the virtual Gateway.
    #[serde(rename = "privateKey")]
    pub private_key: String,
}

/// A reference to an object that represents an SDS issued certificate
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsCertificateSds {
    /// The certificate trust chain for a certificate issued via SDS cluster
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// A reference to an object that represents the Transport Layer Security (TLS) properties for a listener.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VirtualGatewayListenersTlsMode {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "PERMISSIVE")]
    Permissive,
    #[serde(rename = "STRICT")]
    Strict,
}

/// A reference to an object that represents Validation context
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidation {
    /// Possible alternate names to consider
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAlternativeNames")]
    pub subject_alternative_names: Option<VirtualGatewayListenersTlsValidationSubjectAlternativeNames>,
    pub trust: VirtualGatewayListenersTlsValidationTrust,
}

/// Possible alternate names to consider
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationSubjectAlternativeNames {
    /// Match is a required field
    #[serde(rename = "match")]
    pub r#match: VirtualGatewayListenersTlsValidationSubjectAlternativeNamesMatch,
}

/// Match is a required field
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationSubjectAlternativeNamesMatch {
    /// Exact is a required field
    pub exact: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationTrust {
    /// A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acm: Option<VirtualGatewayListenersTlsValidationTrustAcm>,
    /// An object that represents a TLS validation context trust for a local file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayListenersTlsValidationTrustFile>,
    /// An object that represents a TLS validation context trust for an SDS system
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sds: Option<VirtualGatewayListenersTlsValidationTrustSds>,
}

/// A reference to an object that represents a TLS validation context trust for an AWS Certicate Manager (ACM) certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationTrustAcm {
    /// One or more ACM Amazon Resource Name (ARN)s.
    #[serde(rename = "certificateAuthorityARNs")]
    pub certificate_authority_ar_ns: Vec<String>,
}

/// An object that represents a TLS validation context trust for a local file.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationTrustFile {
    /// The certificate trust chain for a certificate stored on the file system of the virtual Gateway.
    #[serde(rename = "certificateChain")]
    pub certificate_chain: String,
}

/// An object that represents a TLS validation context trust for an SDS system
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayListenersTlsValidationTrustSds {
    /// The certificate trust chain for a certificate issued via SDS.
    #[serde(rename = "secretName")]
    pub secret_name: String,
}

/// The inbound and outbound access logging information for the virtual gateway.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayLogging {
    /// The access log configuration for a virtual Gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessLog")]
    pub access_log: Option<VirtualGatewayLoggingAccessLog>,
}

/// The access log configuration for a virtual Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayLoggingAccessLog {
    /// The file object to send virtual gateway access logs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<VirtualGatewayLoggingAccessLogFile>,
}

/// The file object to send virtual gateway access logs to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayLoggingAccessLogFile {
    /// Structured access log output format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<VirtualGatewayLoggingAccessLogFileFormat>,
    /// The file path to write access logs to.
    pub path: String,
}

/// Structured access log output format
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayLoggingAccessLogFileFormat {
    /// Output specified fields as a JSON object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<Vec<VirtualGatewayLoggingAccessLogFileFormatJson>>,
    /// Custom format string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayLoggingAccessLogFileFormatJson {
    /// The name of the field in the JSON object
    pub key: String,
    /// The format string
    pub value: String,
}

/// A reference to k8s Mesh CR that this VirtualGateway belongs to. The admission controller populates it using Meshes's selector, and prevents users from setting this field. 
///  Populated by the system. Read-only.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayMeshRef {
    /// Name is the name of Mesh CR
    pub name: String,
    /// UID is the UID of Mesh CR
    pub uid: String,
}

/// NamespaceSelector selects Namespaces using labels to designate GatewayRoute membership. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayNamespaceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VirtualGatewayNamespaceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayNamespaceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// PodSelector selects Pods using labels to designate VirtualGateway membership. This field follows standard label selector semantics: 	if present but empty, it selects all pods within namespace. 	if absent, it selects no pod.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayPodSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<VirtualGatewayPodSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayPodSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// VirtualGatewayStatus defines the observed state of VirtualGateway
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VirtualGatewayStatus {
    /// The current VirtualGateway status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The generation observed by the VirtualGateway controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// VirtualGatewayARN is the AppMesh VirtualGateway object's Amazon Resource Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualGatewayARN")]
    pub virtual_gateway_arn: Option<String>,
}

