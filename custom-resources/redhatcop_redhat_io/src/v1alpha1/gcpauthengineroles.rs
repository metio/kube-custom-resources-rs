// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/gcpauthengineroles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GCPAuthEngineRoleSpec defines the desired state of GCPAuthEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "GCPAuthEngineRole", plural = "gcpauthengineroles")]
#[kube(namespaced)]
#[kube(status = "GCPAuthEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GCPAuthEngineRoleSpec {
    /// If true, any auth token generated under this token will have associated group aliases, namely project-$PROJECT_ID, folder-$PROJECT_ID, and organization-$ORG_ID for the entities project and all its folder or organization ancestors. This requires Vault to have IAM permission resourcemanager.projects.get.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addGroupAliases")]
    pub add_group_aliases: Option<bool>,
    /// A flag to determine if this role should allow GCE instances to authenticate by inferring service accounts from the GCE identity metadata token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowGCEInference")]
    pub allow_gce_inference: Option<bool>,
    /// Authentication is the kube auth configuraiton to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<GCPAuthEngineRoleAuthentication>,
    /// The instance groups that an authorized instance must belong to in order to be authenticated. If specified, either bound_zones or bound_regions must be set too. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundInstanceGroups")]
    pub bound_instance_groups: Option<Vec<String>>,
    /// A comma-separated list of GCP labels formatted as "key:value" strings that must be set on authorized GCE instances. Because GCP labels are not currently ACL'd, we recommend that this be used in conjunction with other restrictions. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundLabels")]
    pub bound_labels: Option<Vec<String>>,
    /// An array of GCP project IDs. Only entities belonging to this project can authenticate under the role.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundProjects")]
    pub bound_projects: Option<Vec<String>>,
    /// The list of regions that a GCE instance must belong to in order to be authenticated. If bound_instance_groups is provided, it is assumed to be a regional group and the group must belong to this region. If bound_zones are provided, this attribute is ignored. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundRegions")]
    pub bound_regions: Option<Vec<String>>,
    /// An array of service account emails or IDs that login is restricted to, either directly or through an associated instance. If set to *, all service accounts are allowed (you can bind this further using bound_projects.)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundServiceAccounts")]
    pub bound_service_accounts: Option<Vec<String>>,
    /// The list of zones that a GCE instance must belong to in order to be authenticated. If bound_instance_groups is provided, it is assumed to be a zonal group and the group must belong to this zone. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundZones")]
    pub bound_zones: Option<Vec<String>>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<GCPAuthEngineRoleConnection>,
    /// The number of seconds past the time of authentication that the login param JWT must expire within. For example, if a user attempts to login with a token that expires within an hour and this is set to 15 minutes, Vault will return an error prompting the user to create a new signed JWT with a shorter exp. The GCE metadata tokens currently do not allow the exp claim to be customized. The following parameter is only valid when the role is of type "iam".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxJWTExp")]
    pub max_jwt_exp: Option<String>,
    /// Name of the role.
    pub name: String,
    /// Path at which to make the configuration. The final path in Vault will be {[spec.authentication.namespace]}/auth/{spec.path}/groups/{metadata.name}. The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// DEPRECATED: Please use the token_policies parameter instead. List of token policies to encode onto generated tokens. Depending on the auth method, this list may be supplemented by user/group/other values. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    /// List of CIDR blocks. If set, specifies blocks of IP addresses which can authenticate successfully, and ties the resulting token to these blocks as well. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenBoundCIDRs")]
    pub token_bound_cid_rs: Option<Vec<String>>,
    /// If set, will encode an explicit max TTL onto the token. This is a hard cap even if token_ttl and token_max_ttl would otherwise allow a renewal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenExplicitMaxTTL")]
    pub token_explicit_max_ttl: Option<String>,
    /// The maximum lifetime for generated tokens. This current value of this will be referenced at renewal time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenMaxTTL")]
    pub token_max_ttl: Option<String>,
    /// If set, the default policy will not be set on generated tokens; otherwise it will be added to the policies set in token_policies.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenNoDefaultPolicy")]
    pub token_no_default_policy: Option<bool>,
    /// The maximum number of times a generated token may be used (within its lifetime); 0 means unlimited. If you require the token to have the ability to create child tokens, you will need to set this value to 0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenNumUses")]
    pub token_num_uses: Option<i64>,
    /// The maximum allowed period value when a periodic token is requested from this role.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenPeriod")]
    pub token_period: Option<i64>,
    /// List of token policies to encode onto generated tokens. Depending on the auth method, this list may be supplemented by user/group/other values. kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenPolicies")]
    pub token_policies: Option<Vec<String>>,
    /// The incremental lifetime for generated tokens. This current value of this will be referenced at renewal time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenTTL")]
    pub token_ttl: Option<String>,
    /// The type of token that should be generated. Can be service, batch, or default to use the mount's tuned default (which unless changed will be service tokens). For token store roles, there are two additional possibilities: default-service and default-batch which specify the type to return unless the client requests a different type at generation time. For machine based authentication cases, you should use batch type tokens.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenType")]
    pub token_type: Option<String>,
    /// The type of this role. Certain fields correspond to specific roles and will be rejected otherwise. Please see below for more information.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Authentication is the kube auth configuraiton to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleAuthentication {
    /// Namespace is the Vault namespace to be used in all the operations withing this connection/authentication. Only available in Vault Enterprise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Path is the path of the role used for this kube auth authentication. The operator will try to authenticate at {[namespace/]}auth/{spec.path}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Role the role to be used during authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// ServiceAccount is the service account used for the kube auth authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<GCPAuthEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleAuthenticationServiceAccount {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<GCPAuthEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<GCPAuthEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// GCPAuthEngineRoleStatus defines the observed state of GCPAuthEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPAuthEngineRoleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

