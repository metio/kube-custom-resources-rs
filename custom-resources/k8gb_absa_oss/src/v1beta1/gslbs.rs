// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/k8gb-io/k8gb/k8gb.absa.oss/v1beta1/gslbs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// GslbSpec defines the desired state of Gslb
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "k8gb.absa.oss", version = "v1beta1", kind = "Gslb", plural = "gslbs")]
#[kube(namespaced)]
#[kube(status = "GslbStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GslbSpec {
    /// Gslb-enabled Ingress Spec
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<GslbIngress>,
    /// ResourceRef spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRef")]
    pub resource_ref: Option<GslbResourceRef>,
    /// Gslb Strategy spec
    pub strategy: GslbStrategy,
}

/// Gslb-enabled Ingress Spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngress {
    /// A default backend capable of servicing requests that don't match any
    /// rule. At least one of 'backend' or 'rules' must be specified. This field
    /// is optional to allow the loadbalancer controller or defaulting logic to
    /// specify a global default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<GslbIngressBackend>,
    /// IngressClassName is the name of the IngressClass cluster resource. The
    /// associated IngressClass defines which controller will implement the
    /// resource. This replaces the deprecated `kubernetes.io/ingress.class`
    /// annotation. For backwards compatibility, when that annotation is set, it
    /// must be given precedence over this field. The controller may emit a
    /// warning if the field and annotation have different values.
    /// Implementations of this API should ignore Ingresses without a class
    /// specified. An IngressClass resource may be marked as default, which can
    /// be used to set a default value for this field. For more information,
    /// refer to the IngressClass documentation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    /// A list of host rules used to configure the Ingress. If unspecified, or
    /// no rule matches, all traffic is sent to the default backend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<GslbIngressRules>>,
    /// TLS configuration. Currently the Ingress only supports a single TLS
    /// port, 443. If multiple members of this list specify different hosts, they
    /// will be multiplexed on the same port according to the hostname specified
    /// through the SNI TLS extension, if the ingress controller fulfilling the
    /// ingress supports SNI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Vec<GslbIngressTls>>,
}

/// A default backend capable of servicing requests that don't match any
/// rule. At least one of 'backend' or 'rules' must be specified. This field
/// is optional to allow the loadbalancer controller or defaulting logic to
/// specify a global default.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressBackend {
    /// resource is an ObjectRef to another Kubernetes resource in the namespace
    /// of the Ingress object. If resource is specified, a service.Name and
    /// service.Port must not be specified.
    /// This is a mutually exclusive setting with "Service".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<GslbIngressBackendResource>,
    /// service references a service as a backend.
    /// This is a mutually exclusive setting with "Resource".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<GslbIngressBackendService>,
}

/// resource is an ObjectRef to another Kubernetes resource in the namespace
/// of the Ingress object. If resource is specified, a service.Name and
/// service.Port must not be specified.
/// This is a mutually exclusive setting with "Service".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressBackendResource {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// service references a service as a backend.
/// This is a mutually exclusive setting with "Resource".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressBackendService {
    /// name is the referenced service. The service must exist in
    /// the same namespace as the Ingress object.
    pub name: String,
    /// port of the referenced service. A port name or port number
    /// is required for a IngressServiceBackend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<GslbIngressBackendServicePort>,
}

/// port of the referenced service. A port name or port number
/// is required for a IngressServiceBackend.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressBackendServicePort {
    /// name is the name of the port on the Service.
    /// This is a mutually exclusive setting with "Number".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// number is the numerical port number (e.g. 80) on the Service.
    /// This is a mutually exclusive setting with "Name".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

/// IngressRule represents the rules mapping the paths under a specified host to
/// the related backend services. Incoming requests are first evaluated for a host
/// match, then routed to the backend associated with the matching IngressRuleValue.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRules {
    /// Host is the fully qualified domain name of a network host, as defined by RFC 3986.
    /// Note the following deviations from the "host" part of the
    /// URI as defined in RFC 3986:
    /// 1. IPs are not allowed. Currently an IngressRuleValue can only apply to
    ///    the IP in the Spec of the parent Ingress.
    /// 2. The `:` delimiter is not respected because ports are not allowed.
    /// 	  Currently the port of an Ingress is implicitly :80 for http and
    /// 	  :443 for https.
    /// Both these may change in the future.
    /// Incoming requests are matched against the host before the
    /// IngressRuleValue. If the host is unspecified, the Ingress routes all
    /// traffic based on the specified IngressRuleValue.
    /// 
    /// Host can be "precise" which is a domain name without the terminating dot of
    /// a network host (e.g. "foo.bar.com") or "wildcard", which is a domain name
    /// prefixed with a single wildcard label (e.g. "*.foo.com").
    /// The wildcard character '*' must appear by itself as the first DNS label and
    /// matches only a single label. You cannot have a wildcard label by itself (e.g. Host == "*").
    /// Requests will be matched against the Host field in the following way:
    /// 1. If Host is precise, the request matches this rule if the http host header is equal to Host.
    /// 2. If Host is a wildcard, then the request matches this rule if the http host header
    /// is to equal to the suffix (removing the first label) of the wildcard rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// HTTPIngressRuleValue is a list of http selectors
    /// pointing to backends. In the example: http://<host>/<path>?<searchpart>
    /// -> backend where where parts of the url correspond to
    /// RFC 3986, this resource will be used to match against
    /// everything after the last '/' and before the first '?'
    /// or '#'.
    pub http: GslbIngressRulesHttp,
}

/// HTTPIngressRuleValue is a list of http selectors
/// pointing to backends. In the example: http://<host>/<path>?<searchpart>
/// -> backend where where parts of the url correspond to
/// RFC 3986, this resource will be used to match against
/// everything after the last '/' and before the first '?'
/// or '#'.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttp {
    /// paths is a collection of paths that map requests to backends.
    pub paths: Vec<GslbIngressRulesHttpPaths>,
}

/// HTTPIngressPath associates a path with a backend. Incoming urls matching the
/// path are forwarded to the backend.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttpPaths {
    /// backend defines the referenced service endpoint to which the traffic
    /// will be forwarded to.
    pub backend: GslbIngressRulesHttpPathsBackend,
    /// path is matched against the path of an incoming request. Currently it can
    /// contain characters disallowed from the conventional "path" part of a URL
    /// as defined by RFC 3986. Paths must begin with a '/' and must be present
    /// when using PathType with value "Exact" or "Prefix".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// pathType determines the interpretation of the path matching. PathType can
    /// be one of the following values:
    /// * Exact: Matches the URL path exactly.
    /// * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types.
    #[serde(rename = "pathType")]
    pub path_type: String,
}

/// backend defines the referenced service endpoint to which the traffic
/// will be forwarded to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttpPathsBackend {
    /// resource is an ObjectRef to another Kubernetes resource in the namespace
    /// of the Ingress object. If resource is specified, a service.Name and
    /// service.Port must not be specified.
    /// This is a mutually exclusive setting with "Service".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<GslbIngressRulesHttpPathsBackendResource>,
    /// service references a service as a backend.
    /// This is a mutually exclusive setting with "Resource".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<GslbIngressRulesHttpPathsBackendService>,
}

/// resource is an ObjectRef to another Kubernetes resource in the namespace
/// of the Ingress object. If resource is specified, a service.Name and
/// service.Port must not be specified.
/// This is a mutually exclusive setting with "Service".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttpPathsBackendResource {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// service references a service as a backend.
/// This is a mutually exclusive setting with "Resource".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttpPathsBackendService {
    /// name is the referenced service. The service must exist in
    /// the same namespace as the Ingress object.
    pub name: String,
    /// port of the referenced service. A port name or port number
    /// is required for a IngressServiceBackend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<GslbIngressRulesHttpPathsBackendServicePort>,
}

/// port of the referenced service. A port name or port number
/// is required for a IngressServiceBackend.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressRulesHttpPathsBackendServicePort {
    /// name is the name of the port on the Service.
    /// This is a mutually exclusive setting with "Number".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// number is the numerical port number (e.g. 80) on the Service.
    /// This is a mutually exclusive setting with "Name".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

/// IngressTLS describes the transport layer security associated with an ingress.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbIngressTls {
    /// hosts is a list of hosts included in the TLS certificate. The values in
    /// this list must match the name/s used in the tlsSecret. Defaults to the
    /// wildcard host setting for the loadbalancer controller fulfilling this
    /// Ingress, if left unspecified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    /// secretName is the name of the secret used to terminate TLS traffic on
    /// port 443. Field is left optional to allow TLS routing based on SNI
    /// hostname alone. If the SNI host in a listener conflicts with the "Host"
    /// header field used by an IngressRule, the SNI host is used for termination
    /// and value of the "Host" header is used for routing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
}

/// ResourceRef spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbResourceRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<GslbResourceRefMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbResourceRefMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Gslb Strategy spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbStrategy {
    /// Defines DNS record TTL in seconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dnsTtlSeconds")]
    pub dns_ttl_seconds: Option<i64>,
    /// Primary Geo Tag. Valid for failover strategy only
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "primaryGeoTag")]
    pub primary_geo_tag: Option<String>,
    /// Split brain TXT record expiration in seconds. The field is deprecated and not used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "splitBrainThresholdSeconds")]
    pub split_brain_threshold_seconds: Option<i64>,
    /// Load balancing strategy type:(roundRobin|failover)
    #[serde(rename = "type")]
    pub r#type: String,
    /// Weight is defined by map region:weight
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<BTreeMap<String, i64>>,
}

/// GslbStatus defines the observed state of Gslb
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbStatus {
    /// Cluster Geo Tag
    #[serde(rename = "geoTag")]
    pub geo_tag: String,
    /// Current Healthy DNS record structure
    #[serde(rename = "healthyRecords")]
    pub healthy_records: BTreeMap<String, String>,
    /// Comma-separated list of hosts
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<String>,
    /// LoadBalancer configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancer")]
    pub load_balancer: Option<GslbStatusLoadBalancer>,
    /// Servers configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<GslbStatusServers>>,
    /// Associated Service status
    #[serde(rename = "serviceHealth")]
    pub service_health: BTreeMap<String, String>,
}

/// LoadBalancer configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbStatusLoadBalancer {
    /// ExposedIPs on the local Load Balancer
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposedIps")]
    pub exposed_ips: Option<Vec<String>>,
}

/// Servers holds the GSLB's servers' configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbStatusServers {
    /// Hostname exposed by the GSLB
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Kubernetes Services backing the load balanced application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<GslbStatusServersServices>>,
}

/// NamespacedName holds a reference to a k8s resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GslbStatusServersServices {
    /// Name of the resource
    pub name: String,
    /// Namespace where the resource can be found
    pub namespace: String,
}

