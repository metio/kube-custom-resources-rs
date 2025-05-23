// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/aws-application-networking-k8s/application-networking.k8s.aws/v1alpha1/serviceimports.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// spec defines the behavior of a ServiceImport.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "application-networking.k8s.aws", version = "v1alpha1", kind = "ServiceImport", plural = "serviceimports")]
#[kube(namespaced)]
#[kube(status = "ServiceImportStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ServiceImportSpec {
    /// ip will be used as the VIP for this service when type is ClusterSetIP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<String>>,
    pub ports: Vec<ServiceImportPorts>,
    /// Supports "ClientIP" and "None". Used to maintain session affinity.
    /// Enable client IP based session affinity.
    /// Must be ClientIP or None.
    /// Defaults to None.
    /// Ignored when type is Headless
    /// More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionAffinity")]
    pub session_affinity: Option<String>,
    /// sessionAffinityConfig contains session affinity configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionAffinityConfig")]
    pub session_affinity_config: Option<ServiceImportSessionAffinityConfig>,
    /// type defines the type of this service.
    /// Must be ClusterSetIP or Headless.
    #[serde(rename = "type")]
    pub r#type: ServiceImportType,
}

/// ServicePort represents the port on which the service is exposed
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceImportPorts {
    /// The application protocol for this port.
    /// This field follows standard Kubernetes label syntax.
    /// Un-prefixed names are reserved for IANA standard service names (as per
    /// RFC-6335 and http://www.iana.org/assignments/service-names).
    /// Non-standard protocols should use prefixed names such as
    /// mycompany.com/my-custom-protocol.
    /// Field can be enabled with ServiceAppProtocol feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appProtocol")]
    pub app_protocol: Option<String>,
    /// The name of this port within the service. This must be a DNS_LABEL.
    /// All ports within a ServiceSpec must have unique names. When considering
    /// the endpoints for a Service, this must match the 'name' field in the
    /// EndpointPort.
    /// Optional if only one ServicePort is defined on this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The port that will be exposed by this service.
    pub port: i32,
    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP".
    /// Default is TCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// sessionAffinityConfig contains session affinity configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceImportSessionAffinityConfig {
    /// clientIP contains the configurations of Client IP based session affinity.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientIP")]
    pub client_ip: Option<ServiceImportSessionAffinityConfigClientIp>,
}

/// clientIP contains the configurations of Client IP based session affinity.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceImportSessionAffinityConfigClientIp {
    /// timeoutSeconds specifies the seconds of ClientIP type session sticky time.
    /// The value must be >0 && <=86400(for 1 day) if ServiceAffinity == "ClientIP".
    /// Default value is 10800(for 3 hours).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeoutSeconds")]
    pub timeout_seconds: Option<i32>,
}

/// spec defines the behavior of a ServiceImport.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ServiceImportType {
    #[serde(rename = "ClusterSetIP")]
    ClusterSetIp,
    Headless,
}

/// status contains information about the exported services that form
/// the multi-cluster service referenced by this ServiceImport.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceImportStatus {
    /// clusters is the list of exporting clusters from which this service
    /// was derived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ServiceImportStatusClusters>>,
}

/// ClusterStatus contains service configuration mapped to a specific source cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ServiceImportStatusClusters {
    /// cluster is the name of the exporting cluster. Must be a valid RFC-1123 DNS
    /// label.
    pub cluster: String,
}

