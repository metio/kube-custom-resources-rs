// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/l7mp/stunner/stunner.l7mp.io/v1alpha1/staticservices.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Spec defines the behavior of a service.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "stunner.l7mp.io", version = "v1alpha1", kind = "StaticService", plural = "staticservices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct StaticServiceSpec {
    /// The list of ports reachable via this service (currently omitted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<StaticServicePorts>>,
    /// Prefixes is a list of IP address prefixes reachable via this route.
    pub prefixes: Vec<String>,
}

/// ServicePort contains information on service's port.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct StaticServicePorts {
    /// The application protocol for this port.
    /// This is used as a hint for implementations to offer richer behavior for protocols that they understand.
    /// This field follows standard Kubernetes label syntax.
    /// Valid values are either:
    /// 
    /// 
    /// * Un-prefixed protocol names - reserved for IANA standard service names (as per
    /// RFC-6335 and https://www.iana.org/assignments/service-names).
    /// 
    /// 
    /// * Kubernetes-defined prefixed names:
    ///   * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
    ///   * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
    ///   * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455
    /// 
    /// 
    /// * Other protocols should use implementation-defined prefixed names such as
    /// mycompany.com/my-custom-protocol.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appProtocol")]
    pub app_protocol: Option<String>,
    /// The name of this port within the service. This must be a DNS_LABEL.
    /// All ports within a ServiceSpec must have unique names. When considering
    /// the endpoints for a Service, this must match the 'name' field in the
    /// EndpointPort.
    /// Optional if only one ServicePort is defined on this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The port on each node on which this service is exposed when type is
    /// NodePort or LoadBalancer.  Usually assigned by the system. If a value is
    /// specified, in-range, and not in use it will be used, otherwise the
    /// operation will fail.  If not specified, a port will be allocated if this
    /// Service requires one.  If this field is specified when creating a
    /// Service which does not need it, creation will fail. This field will be
    /// wiped when updating a Service to no longer need it (e.g. changing type
    /// from NodePort to ClusterIP).
    /// More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodePort")]
    pub node_port: Option<i32>,
    /// The port that will be exposed by this service.
    pub port: i32,
    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP".
    /// Default is TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Number or name of the port to access on the pods targeted by the service.
    /// Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    /// If this is a string, it will be looked up as a named port in the
    /// target Pod's container ports. If this is not specified, the value
    /// of the 'port' field is used (an identity map).
    /// This field is ignored for services with clusterIP=None, and should be
    /// omitted or set equal to the 'port' field.
    /// More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<IntOrString>,
}

