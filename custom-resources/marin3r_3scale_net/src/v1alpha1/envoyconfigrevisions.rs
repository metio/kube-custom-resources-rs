// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/3scale-ops/marin3r/marin3r.3scale.net/v1alpha1/envoyconfigrevisions.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// EnvoyConfigRevisionSpec defines the desired state of EnvoyConfigRevision
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "marin3r.3scale.net", version = "v1alpha1", kind = "EnvoyConfigRevision", plural = "envoyconfigrevisions")]
#[kube(namespaced)]
#[kube(status = "EnvoyConfigRevisionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EnvoyConfigRevisionSpec {
    /// EnvoyAPI is the version of envoy's API to use. Defaults to v3.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envoyAPI")]
    pub envoy_api: Option<EnvoyConfigRevisionEnvoyApi>,
    /// EnvoyResources holds the different types of resources suported by the envoy discovery service
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envoyResources")]
    pub envoy_resources: Option<EnvoyConfigRevisionEnvoyResources>,
    /// NodeID holds the envoy identifier for the discovery service to know which set
    /// of resources to send to each of the envoy clients that connect to it.
    #[serde(rename = "nodeID")]
    pub node_id: String,
    /// Resources holds the different types of resources suported by the envoy discovery service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<EnvoyConfigRevisionResources>>,
    /// Serialization specicifies the serialization format used to describe the resources. "json" and "yaml"
    /// are supported. "json" is used if unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serialization: Option<EnvoyConfigRevisionSerialization>,
    /// Version is a hash of the EnvoyResources field
    pub version: String,
}

/// EnvoyConfigRevisionSpec defines the desired state of EnvoyConfigRevision
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyConfigRevisionEnvoyApi {
    #[serde(rename = "v3")]
    V3,
}

/// EnvoyResources holds the different types of resources suported by the envoy discovery service
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResources {
    /// Clusters is a list of the envoy Cluster resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/cluster/v3/cluster.proto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<EnvoyConfigRevisionEnvoyResourcesClusters>>,
    /// Endpoints is a list of the envoy ClusterLoadAssignment resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/endpoint/v3/endpoint.proto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<EnvoyConfigRevisionEnvoyResourcesEndpoints>>,
    /// ExtensionConfigs is a list of the envoy ExtensionConfig resource type
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/core/v3/extension.proto
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionConfigs")]
    pub extension_configs: Option<Vec<EnvoyConfigRevisionEnvoyResourcesExtensionConfigs>>,
    /// Listeners is a list of the envoy Listener resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/listener/v3/listener.proto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<EnvoyConfigRevisionEnvoyResourcesListeners>>,
    /// Routes is a list of the envoy Route resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/route/v3/route.proto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<EnvoyConfigRevisionEnvoyResourcesRoutes>>,
    /// Runtimes is a list of the envoy Runtime resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/runtime/v3/rtds.proto
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<Vec<EnvoyConfigRevisionEnvoyResourcesRuntimes>>,
    /// ScopedRoutes is a list of the envoy ScopeRoute resource type.
    /// API V3 reference: https://www.envoyproxy.io/docs/envoy/latest/api-v3/config/route/v3/scoped_route.proto
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopedRoutes")]
    pub scoped_routes: Option<Vec<EnvoyConfigRevisionEnvoyResourcesScopedRoutes>>,
    /// Secrets is a list of references to Kubernetes Secret objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<EnvoyConfigRevisionEnvoyResourcesSecrets>>,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesClusters {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesEndpoints {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesExtensionConfigs {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesListeners {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesRoutes {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesRuntimes {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoyResource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesScopedRoutes {
    /// Name of the envoy resource.
    /// DEPRECATED: this field has no effect and will be removed in an
    /// upcoming release. The name of the resources for discovery purposes
    /// is included in the resource itself. Refer to the envoy API reference
    /// to check how the name is specified for each resource type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value is the serialized representation of the envoy resource
    pub value: String,
}

/// EnvoySecretResource holds a reference to a k8s Secret from where
/// to take a secret from. Only Secrets within the same namespace can
/// be referred.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesSecrets {
    /// Name of the envoy tslCerticate secret resource. The certificate will be fetched
    /// from a Kubernetes Secrets of type 'kubernetes.io/tls' with this same name.
    pub name: String,
    /// DEPRECATED: this field is deprecated and it's value will be ignored. The 'name' of the
    /// Kubernetes Secret must match the 'name' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ref")]
    pub r#ref: Option<EnvoyConfigRevisionEnvoyResourcesSecretsRef>,
}

/// DEPRECATED: this field is deprecated and it's value will be ignored. The 'name' of the
/// Kubernetes Secret must match the 'name' field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionEnvoyResourcesSecretsRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Resource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EnvoyConfigRevisionResources {
    /// Blueprint specifies a template to generate a configuration proto. It is currently
    /// only supported to generate secret configuration resources from k8s Secrets
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<EnvoyConfigRevisionResourcesBlueprint>,
    /// Specifies a label selector to watch for EndpointSlices that will
    /// be used to generate the endpoint resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateFromEndpointSlices")]
    pub generate_from_endpoint_slices: Option<EnvoyConfigRevisionResourcesGenerateFromEndpointSlices>,
    /// The name of a Kubernetes Secret of type "Opaque". It will generate an
    /// envoy "generic secret" proto.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateFromOpaqueSecret")]
    pub generate_from_opaque_secret: Option<EnvoyConfigRevisionResourcesGenerateFromOpaqueSecret>,
    /// The name of a Kubernetes Secret of type "kubernetes.io/tls"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "generateFromTlsSecret")]
    pub generate_from_tls_secret: Option<String>,
    /// Type is the type url for the protobuf message
    #[serde(rename = "type")]
    pub r#type: EnvoyConfigRevisionResourcesType,
    /// Value is the protobufer message that configures the resource. The proto
    /// must match the envoy configuration API v3 specification for the given resource
    /// type (https://www.envoyproxy.io/docs/envoy/latest/api-docs/xds_protocol#resource-types)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, serde_json::Value>>,
}

/// Resource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyConfigRevisionResourcesBlueprint {
    #[serde(rename = "tlsCertificate")]
    TlsCertificate,
    #[serde(rename = "validationContext")]
    ValidationContext,
}

/// Specifies a label selector to watch for EndpointSlices that will
/// be used to generate the endpoint resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionResourcesGenerateFromEndpointSlices {
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null
    /// label selector matches no objects.
    pub selector: EnvoyConfigRevisionResourcesGenerateFromEndpointSlicesSelector,
    #[serde(rename = "targetPort")]
    pub target_port: String,
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null
/// label selector matches no objects.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionResourcesGenerateFromEndpointSlicesSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<EnvoyConfigRevisionResourcesGenerateFromEndpointSlicesSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionResourcesGenerateFromEndpointSlicesSelectorMatchExpressions {
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

/// The name of a Kubernetes Secret of type "Opaque". It will generate an
/// envoy "generic secret" proto.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionResourcesGenerateFromOpaqueSecret {
    /// A unique name to refer to the name:key combination
    pub alias: String,
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// The name of the secret in the pod's namespace to select from.
    pub name: String,
}

/// Resource holds serialized representation of an envoy
/// resource
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyConfigRevisionResourcesType {
    #[serde(rename = "listener")]
    Listener,
    #[serde(rename = "route")]
    Route,
    #[serde(rename = "scopedRoute")]
    ScopedRoute,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "endpoint")]
    Endpoint,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "runtime")]
    Runtime,
    #[serde(rename = "extensionConfig")]
    ExtensionConfig,
}

/// EnvoyConfigRevisionSpec defines the desired state of EnvoyConfigRevision
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EnvoyConfigRevisionSerialization {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "b64json")]
    B64json,
    #[serde(rename = "yaml")]
    Yaml,
}

/// EnvoyConfigRevisionStatus defines the observed state of EnvoyConfigRevision
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastPublishedAt indicates the last time this config review transitioned to
    /// published
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastPublishedAt")]
    pub last_published_at: Option<String>,
    /// ProvidesVersions keeps track of the version that this revision
    /// publishes in the xDS server for each resource type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providesVersions")]
    pub provides_versions: Option<EnvoyConfigRevisionStatusProvidesVersions>,
    /// Published signals if the EnvoyConfigRevision is the one currently published
    /// in the xds server cache
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// Tainted indicates whether the EnvoyConfigRevision is eligible for publishing
    /// or not
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tainted: Option<bool>,
}

/// ProvidesVersions keeps track of the version that this revision
/// publishes in the xDS server for each resource type
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EnvoyConfigRevisionStatusProvidesVersions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionConfigs")]
    pub extension_configs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scopedRoutes")]
    pub scoped_routes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<String>,
}

