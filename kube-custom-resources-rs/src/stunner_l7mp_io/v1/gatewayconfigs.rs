// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/l7mp/stunner/stunner.l7mp.io/v1/gatewayconfigs.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// GatewayConfigSpec defines the desired state of GatewayConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "stunner.l7mp.io", version = "v1", kind = "GatewayConfig", plural = "gatewayconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GatewayConfigSpec {
    /// AuthLifetime defines the lifetime of "longterm" authentication credentials in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authLifetime")]
    pub auth_lifetime: Option<i32>,
    /// Note that externally set credentials override any inline auth credentials (AuthType,
    /// AuthUsername, etc.): if AuthRef is nonempty then it is expected that the referenced
    /// Secret exists and *all* authentication credentials are correctly set in the referenced
    /// Secret (username/password or shared secret). Mixing of credential sources
    /// (inline/external) is not supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authRef")]
    pub auth_ref: Option<GatewayConfigAuthRef>,
    /// AuthType is the type of the STUN/TURN authentication mechanism.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authType")]
    pub auth_type: Option<String>,
    /// Dataplane defines the dataplane (stunnerd image, version, etc) for STUNner gateways
    /// using this GatewayConfig.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataplane: Option<String>,
    /// LoadBalancerServiceAnnotations is a list of annotations that will go into the
    /// LoadBalancer services created automatically by the operator to wrap Gateways.
    /// 
    /// 
    /// NOTE: removing annotations from a GatewayConfig will not result in the removal of the
    /// corresponding annotations from the LoadBalancer service, in order to prevent the
    /// accidental removal of an annotation installed there by Kubernetes or the cloud
    /// provider. If you really want to remove an annotation, do this manually or simply remove
    /// all Gateways (which will remove the corresponding LoadBalancer services), update the
    /// GatewayConfig and then recreate the Gateways, so that the newly created LoadBalancer
    /// services will contain the required annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerServiceAnnotations")]
    pub load_balancer_service_annotations: Option<BTreeMap<String, String>>,
    /// LogLevel specifies the default loglevel for the STUNner daemon.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<String>,
    /// Password defines the `password` credential for "plaintext" authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Realm defines the STUN/TURN authentication realm to be used for clients toauthenticate
    /// with STUNner.
    /// 
    /// 
    /// The realm must consist of lower case alphanumeric characters or '-', and must start and
    /// end with an alphanumeric character. No other punctuation is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    /// SharedSecret defines the shared secret to be used for "longterm" authentication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sharedSecret")]
    pub shared_secret: Option<String>,
    /// Username defines the `username` credential for "plaintext" authentication.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
}

/// Note that externally set credentials override any inline auth credentials (AuthType,
/// AuthUsername, etc.): if AuthRef is nonempty then it is expected that the referenced
/// Secret exists and *all* authentication credentials are correctly set in the referenced
/// Secret (username/password or shared secret). Mixing of credential sources
/// (inline/external) is not supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GatewayConfigAuthRef {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind is kind of the referent. For example "Secret".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    /// 
    /// 
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    /// 
    /// 
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

