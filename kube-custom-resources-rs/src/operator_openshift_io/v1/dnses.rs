// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/openshift/api/operator.openshift.io/v1/dnses.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// spec is the specification of the desired behavior of the DNS.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "operator.openshift.io", version = "v1", kind = "DNS", plural = "dnses")]
#[kube(status = "DNSStatus")]
#[kube(schema = "disabled")]
pub struct DNSSpec {
    /// cache describes the caching configuration that applies to all server blocks listed in the Corefile. This field allows a cluster admin to optionally configure: * positiveTTL which is a duration for which positive responses should be cached. * negativeTTL which is a duration for which negative responses should be cached. If this is not configured, OpenShift will configure positive and negative caching with a default value that is subject to change. At the time of writing, the default positiveTTL is 900 seconds and the default negativeTTL is 30 seconds or as noted in the respective Corefile for your version of OpenShift.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<DNSCache>,
    /// logLevel describes the desired logging verbosity for CoreDNS. Any one of the following values may be specified: * Normal logs errors from upstream resolvers. * Debug logs errors, NXDOMAIN responses, and NODATA responses. * Trace logs errors and all responses. Setting logLevel: Trace will produce extremely verbose logs. Valid values are: "Normal", "Debug", "Trace". Defaults to "Normal".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<DNSLogLevel>,
    /// managementState indicates whether the DNS operator should manage cluster DNS
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managementState")]
    pub management_state: Option<String>,
    /// nodePlacement provides explicit control over the scheduling of DNS pods. 
    ///  Generally, it is useful to run a DNS pod on every node so that DNS queries are always handled by a local DNS pod instead of going over the network to a DNS pod on another node.  However, security policies may require restricting the placement of DNS pods to specific nodes. For example, if a security policy prohibits pods on arbitrary nodes from communicating with the API, a node selector can be specified to restrict DNS pods to nodes that are permitted to communicate with the API.  Conversely, if running DNS pods on nodes with a particular taint is desired, a toleration can be specified for that taint. 
    ///  If unset, defaults are used. See nodePlacement for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodePlacement")]
    pub node_placement: Option<DNSNodePlacement>,
    /// operatorLogLevel controls the logging level of the DNS Operator. Valid values are: "Normal", "Debug", "Trace". Defaults to "Normal". setting operatorLogLevel: Trace will produce extremely verbose logs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operatorLogLevel")]
    pub operator_log_level: Option<DNSOperatorLogLevel>,
    /// servers is a list of DNS resolvers that provide name query delegation for one or more subdomains outside the scope of the cluster domain. If servers consists of more than one Server, longest suffix match will be used to determine the Server. 
    ///  For example, if there are two Servers, one for "foo.com" and another for "a.foo.com", and the name query is for "www.a.foo.com", it will be routed to the Server with Zone "a.foo.com". 
    ///  If this field is nil, no servers are created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<DNSServers>>,
    /// upstreamResolvers defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers for the case of the default (".") server 
    ///  If this field is not specified, the upstream used will default to /etc/resolv.conf, with policy "sequential"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamResolvers")]
    pub upstream_resolvers: Option<DNSUpstreamResolvers>,
}

/// cache describes the caching configuration that applies to all server blocks listed in the Corefile. This field allows a cluster admin to optionally configure: * positiveTTL which is a duration for which positive responses should be cached. * negativeTTL which is a duration for which negative responses should be cached. If this is not configured, OpenShift will configure positive and negative caching with a default value that is subject to change. At the time of writing, the default positiveTTL is 900 seconds and the default negativeTTL is 30 seconds or as noted in the respective Corefile for your version of OpenShift.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSCache {
    /// negativeTTL is optional and specifies the amount of time that a negative response should be cached. 
    ///  If configured, it must be a value of 1s (1 second) or greater up to a theoretical maximum of several years. This field expects an unsigned duration string of decimal numbers, each with optional fraction and a unit suffix, e.g. "100s", "1m30s", "12h30m10s". Values that are fractions of a second are rounded down to the nearest second. If the configured value is less than 1s, the default value will be used. If not configured, the value will be 0s and OpenShift will use a default value of 30 seconds unless noted otherwise in the respective Corefile for your version of OpenShift. The default value of 30 seconds is subject to change.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "negativeTTL")]
    pub negative_ttl: Option<String>,
    /// positiveTTL is optional and specifies the amount of time that a positive response should be cached. 
    ///  If configured, it must be a value of 1s (1 second) or greater up to a theoretical maximum of several years. This field expects an unsigned duration string of decimal numbers, each with optional fraction and a unit suffix, e.g. "100s", "1m30s", "12h30m10s". Values that are fractions of a second are rounded down to the nearest second. If the configured value is less than 1s, the default value will be used. If not configured, the value will be 0s and OpenShift will use a default value of 900 seconds unless noted otherwise in the respective Corefile for your version of OpenShift. The default value of 900 seconds is subject to change.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "positiveTTL")]
    pub positive_ttl: Option<String>,
}

/// spec is the specification of the desired behavior of the DNS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSLogLevel {
    Normal,
    Debug,
    Trace,
}

/// nodePlacement provides explicit control over the scheduling of DNS pods. 
///  Generally, it is useful to run a DNS pod on every node so that DNS queries are always handled by a local DNS pod instead of going over the network to a DNS pod on another node.  However, security policies may require restricting the placement of DNS pods to specific nodes. For example, if a security policy prohibits pods on arbitrary nodes from communicating with the API, a node selector can be specified to restrict DNS pods to nodes that are permitted to communicate with the API.  Conversely, if running DNS pods on nodes with a particular taint is desired, a toleration can be specified for that taint. 
///  If unset, defaults are used. See nodePlacement for more details.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSNodePlacement {
    /// nodeSelector is the node selector applied to DNS pods. 
    ///  If empty, the default is used, which is currently the following: 
    ///  kubernetes.io/os: linux 
    ///  This default is subject to change. 
    ///  If set, the specified selector is used and replaces the default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<BTreeMap<String, String>>,
    /// tolerations is a list of tolerations applied to DNS pods. 
    ///  If empty, the DNS operator sets a toleration for the "node-role.kubernetes.io/master" taint.  This default is subject to change.  Specifying tolerations without including a toleration for the "node-role.kubernetes.io/master" taint may be risky as it could lead to an outage if all worker nodes become unavailable. 
    ///  Note that the daemon controller adds some tolerations as well.  See https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<DNSNodePlacementTolerations>>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSNodePlacementTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// spec is the specification of the desired behavior of the DNS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSOperatorLogLevel {
    Normal,
    Debug,
    Trace,
}

/// Server defines the schema for a server that runs per instance of CoreDNS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSServers {
    /// forwardPlugin defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardPlugin")]
    pub forward_plugin: Option<DNSServersForwardPlugin>,
    /// name is required and specifies a unique name for the server. Name must comply with the Service Name Syntax of rfc6335.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// zones is required and specifies the subdomains that Server is authoritative for. Zones must conform to the rfc1123 definition of a subdomain. Specifying the cluster domain (i.e., "cluster.local") is invalid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// forwardPlugin defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSServersForwardPlugin {
    /// policy is used to determine the order in which upstream servers are selected for querying. Any one of the following values may be specified: 
    ///  * "Random" picks a random upstream server for each query. * "RoundRobin" picks upstream servers in a round-robin order, moving to the next server for each new query. * "Sequential" tries querying upstream servers in a sequential order until one responds, starting with the first server for each new query. 
    ///  The default value is "Random"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<DNSServersForwardPluginPolicy>,
    /// protocolStrategy specifies the protocol to use for upstream DNS requests. Valid values for protocolStrategy are "TCP" and omitted. When omitted, this means no opinion and the platform is left to choose a reasonable default, which is subject to change over time. The current default is to use the protocol of the original client request. "TCP" specifies that the platform should use TCP for all upstream DNS requests, even if the client request uses UDP. "TCP" is useful for UDP-specific issues such as those created by non-compliant upstream resolvers, but may consume more bandwidth or increase DNS response time. Note that protocolStrategy only affects the protocol of DNS requests that CoreDNS makes to upstream resolvers. It does not affect the protocol of DNS requests between clients and CoreDNS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolStrategy")]
    pub protocol_strategy: Option<DNSServersForwardPluginProtocolStrategy>,
    /// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
    ///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transportConfig")]
    pub transport_config: Option<DNSServersForwardPluginTransportConfig>,
    /// upstreams is a list of resolvers to forward name queries for subdomains of Zones. Each instance of CoreDNS performs health checking of Upstreams. When a healthy upstream returns an error during the exchange, another resolver is tried from Upstreams. The Upstreams are selected in the order specified in Policy. Each upstream is represented by an IP address or IP:port if the upstream listens on a port other than 53. 
    ///  A maximum of 15 upstreams is allowed per ForwardPlugin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<String>>,
}

/// forwardPlugin defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSServersForwardPluginPolicy {
    Random,
    RoundRobin,
    Sequential,
}

/// forwardPlugin defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSServersForwardPluginProtocolStrategy {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSServersForwardPluginTransportConfig {
    /// tls contains the additional configuration options to use when Transport is set to "TLS".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<DNSServersForwardPluginTransportConfigTls>,
    /// transport allows cluster administrators to opt-in to using a DNS-over-TLS connection between cluster DNS and an upstream resolver(s). Configuring TLS as the transport at this level without configuring a CABundle will result in the system certificates being used to verify the serving certificate of the upstream resolver(s). 
    ///  Possible values: "" (empty) - This means no explicit choice has been made and the platform chooses the default which is subject to change over time. The current default is "Cleartext". "Cleartext" - Cluster admin specified cleartext option. This results in the same functionality as an empty value but may be useful when a cluster admin wants to be more explicit about the transport, or wants to switch from "TLS" to "Cleartext" explicitly. "TLS" - This indicates that DNS queries should be sent over a TLS connection. If Transport is set to TLS, you MUST also set ServerName. If a port is not included with the upstream IP, port 853 will be tried by default per RFC 7858 section 3.1; https://datatracker.ietf.org/doc/html/rfc7858#section-3.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<DNSServersForwardPluginTransportConfigTransport>,
}

/// tls contains the additional configuration options to use when Transport is set to "TLS".
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSServersForwardPluginTransportConfigTls {
    /// caBundle references a ConfigMap that must contain either a single CA Certificate or a CA Bundle. This allows cluster administrators to provide their own CA or CA bundle for validating the certificate of upstream resolvers. 
    ///  1. The configmap must contain a `ca-bundle.crt` key. 2. The value must be a PEM encoded CA certificate or CA bundle. 3. The administrator must create this configmap in the openshift-config namespace. 4. The upstream server certificate must contain a Subject Alternative Name (SAN) that matches ServerName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caBundle")]
    pub ca_bundle: Option<DNSServersForwardPluginTransportConfigTlsCaBundle>,
    /// serverName is the upstream server to connect to when forwarding DNS queries. This is required when Transport is set to "TLS". ServerName will be validated against the DNS naming conventions in RFC 1123 and should match the TLS certificate installed in the upstream resolver(s).
    #[serde(rename = "serverName")]
    pub server_name: String,
}

/// caBundle references a ConfigMap that must contain either a single CA Certificate or a CA Bundle. This allows cluster administrators to provide their own CA or CA bundle for validating the certificate of upstream resolvers. 
///  1. The configmap must contain a `ca-bundle.crt` key. 2. The value must be a PEM encoded CA certificate or CA bundle. 3. The administrator must create this configmap in the openshift-config namespace. 4. The upstream server certificate must contain a Subject Alternative Name (SAN) that matches ServerName.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSServersForwardPluginTransportConfigTlsCaBundle {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSServersForwardPluginTransportConfigTransport {
    #[serde(rename = "TLS")]
    Tls,
    Cleartext,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// upstreamResolvers defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers for the case of the default (".") server 
///  If this field is not specified, the upstream used will default to /etc/resolv.conf, with policy "sequential"
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSUpstreamResolvers {
    /// Policy is used to determine the order in which upstream servers are selected for querying. Any one of the following values may be specified: 
    ///  * "Random" picks a random upstream server for each query. * "RoundRobin" picks upstream servers in a round-robin order, moving to the next server for each new query. * "Sequential" tries querying upstream servers in a sequential order until one responds, starting with the first server for each new query. 
    ///  The default value is "Sequential"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<DNSUpstreamResolversPolicy>,
    /// protocolStrategy specifies the protocol to use for upstream DNS requests. Valid values for protocolStrategy are "TCP" and omitted. When omitted, this means no opinion and the platform is left to choose a reasonable default, which is subject to change over time. The current default is to use the protocol of the original client request. "TCP" specifies that the platform should use TCP for all upstream DNS requests, even if the client request uses UDP. "TCP" is useful for UDP-specific issues such as those created by non-compliant upstream resolvers, but may consume more bandwidth or increase DNS response time. Note that protocolStrategy only affects the protocol of DNS requests that CoreDNS makes to upstream resolvers. It does not affect the protocol of DNS requests between clients and CoreDNS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolStrategy")]
    pub protocol_strategy: Option<DNSUpstreamResolversProtocolStrategy>,
    /// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
    ///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transportConfig")]
    pub transport_config: Option<DNSUpstreamResolversTransportConfig>,
    /// Upstreams is a list of resolvers to forward name queries for the "." domain. Each instance of CoreDNS performs health checking of Upstreams. When a healthy upstream returns an error during the exchange, another resolver is tried from Upstreams. The Upstreams are selected in the order specified in Policy. 
    ///  A maximum of 15 upstreams is allowed per ForwardPlugin. If no Upstreams are specified, /etc/resolv.conf is used by default
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<DNSUpstreamResolversUpstreams>>,
}

/// upstreamResolvers defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers for the case of the default (".") server 
///  If this field is not specified, the upstream used will default to /etc/resolv.conf, with policy "sequential"
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSUpstreamResolversPolicy {
    Random,
    RoundRobin,
    Sequential,
}

/// upstreamResolvers defines a schema for configuring CoreDNS to proxy DNS messages to upstream resolvers for the case of the default (".") server 
///  If this field is not specified, the upstream used will default to /etc/resolv.conf, with policy "sequential"
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSUpstreamResolversProtocolStrategy {
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSUpstreamResolversTransportConfig {
    /// tls contains the additional configuration options to use when Transport is set to "TLS".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<DNSUpstreamResolversTransportConfigTls>,
    /// transport allows cluster administrators to opt-in to using a DNS-over-TLS connection between cluster DNS and an upstream resolver(s). Configuring TLS as the transport at this level without configuring a CABundle will result in the system certificates being used to verify the serving certificate of the upstream resolver(s). 
    ///  Possible values: "" (empty) - This means no explicit choice has been made and the platform chooses the default which is subject to change over time. The current default is "Cleartext". "Cleartext" - Cluster admin specified cleartext option. This results in the same functionality as an empty value but may be useful when a cluster admin wants to be more explicit about the transport, or wants to switch from "TLS" to "Cleartext" explicitly. "TLS" - This indicates that DNS queries should be sent over a TLS connection. If Transport is set to TLS, you MUST also set ServerName. If a port is not included with the upstream IP, port 853 will be tried by default per RFC 7858 section 3.1; https://datatracker.ietf.org/doc/html/rfc7858#section-3.1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<DNSUpstreamResolversTransportConfigTransport>,
}

/// tls contains the additional configuration options to use when Transport is set to "TLS".
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSUpstreamResolversTransportConfigTls {
    /// caBundle references a ConfigMap that must contain either a single CA Certificate or a CA Bundle. This allows cluster administrators to provide their own CA or CA bundle for validating the certificate of upstream resolvers. 
    ///  1. The configmap must contain a `ca-bundle.crt` key. 2. The value must be a PEM encoded CA certificate or CA bundle. 3. The administrator must create this configmap in the openshift-config namespace. 4. The upstream server certificate must contain a Subject Alternative Name (SAN) that matches ServerName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caBundle")]
    pub ca_bundle: Option<DNSUpstreamResolversTransportConfigTlsCaBundle>,
    /// serverName is the upstream server to connect to when forwarding DNS queries. This is required when Transport is set to "TLS". ServerName will be validated against the DNS naming conventions in RFC 1123 and should match the TLS certificate installed in the upstream resolver(s).
    #[serde(rename = "serverName")]
    pub server_name: String,
}

/// caBundle references a ConfigMap that must contain either a single CA Certificate or a CA Bundle. This allows cluster administrators to provide their own CA or CA bundle for validating the certificate of upstream resolvers. 
///  1. The configmap must contain a `ca-bundle.crt` key. 2. The value must be a PEM encoded CA certificate or CA bundle. 3. The administrator must create this configmap in the openshift-config namespace. 4. The upstream server certificate must contain a Subject Alternative Name (SAN) that matches ServerName.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSUpstreamResolversTransportConfigTlsCaBundle {
    /// name is the metadata.name of the referenced config map
    pub name: String,
}

/// transportConfig is used to configure the transport type, server name, and optional custom CA or CA bundle to use when forwarding DNS requests to an upstream resolver. 
///  The default value is "" (empty) which results in a standard cleartext connection being used when forwarding DNS requests to an upstream resolver.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSUpstreamResolversTransportConfigTransport {
    #[serde(rename = "TLS")]
    Tls,
    Cleartext,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// Upstream can either be of type SystemResolvConf, or of type Network. 
///  * For an Upstream of type SystemResolvConf, no further fields are necessary: The upstream will be configured to use /etc/resolv.conf. * For an Upstream of type Network, a NetworkResolver field needs to be defined with an IP address or IP:port if the upstream listens on a port other than 53.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSUpstreamResolversUpstreams {
    /// Address must be defined when Type is set to Network. It will be ignored otherwise. It must be a valid ipv4 or ipv6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Port may be defined when Type is set to Network. It will be ignored otherwise. Port must be between 65535
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Type defines whether this upstream contains an IP/IP:port resolver or the local /etc/resolv.conf. Type accepts 2 possible values: SystemResolvConf or Network. 
    ///  * When SystemResolvConf is used, the Upstream structure does not require any further fields to be defined: /etc/resolv.conf will be used * When Network is used, the Upstream structure must contain at least an Address
    #[serde(rename = "type")]
    pub r#type: DNSUpstreamResolversUpstreamsType,
}

/// Upstream can either be of type SystemResolvConf, or of type Network. 
///  * For an Upstream of type SystemResolvConf, no further fields are necessary: The upstream will be configured to use /etc/resolv.conf. * For an Upstream of type Network, a NetworkResolver field needs to be defined with an IP address or IP:port if the upstream listens on a port other than 53.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DNSUpstreamResolversUpstreamsType {
    SystemResolvConf,
    Network,
    #[serde(rename = "")]
    KopiumEmpty,
}

/// status is the most recently observed status of the DNS.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSStatus {
    /// clusterDomain is the local cluster DNS domain suffix for DNS services. This will be a subdomain as defined in RFC 1034, section 3.5: https://tools.ietf.org/html/rfc1034#section-3.5 Example: "cluster.local" 
    ///  More info: https://kubernetes.io/docs/concepts/services-networking/dns-pod-service
    #[serde(rename = "clusterDomain")]
    pub cluster_domain: String,
    /// clusterIP is the service IP through which this DNS is made available. 
    ///  In the case of the default DNS, this will be a well known IP that is used as the default nameserver for pods that are using the default ClusterFirst DNS policy. 
    ///  In general, this IP can be specified in a pod's spec.dnsConfig.nameservers list or used explicitly when performing name resolution from within the cluster. Example: dig foo.com @<service IP> 
    ///  More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(rename = "clusterIP")]
    pub cluster_ip: String,
    /// conditions provide information about the state of the DNS on the cluster. 
    ///  These are the supported DNS conditions: 
    ///  * Available - True if the following conditions are met: * DNS controller daemonset is available. - False if any of those conditions are unsatisfied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DNSStatusConditions>>,
}

/// OperatorCondition is just the standard condition fields.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DNSStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

