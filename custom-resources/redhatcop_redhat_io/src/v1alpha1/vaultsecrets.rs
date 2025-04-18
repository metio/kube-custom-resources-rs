// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/vaultsecrets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VaultSecretSpec defines the desired state of VaultSecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "VaultSecret", plural = "vaultsecrets")]
#[kube(namespaced)]
#[kube(status = "VaultSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VaultSecretSpec {
    /// TemplatizedK8sSecret is the formatted K8s Secret created by templating from the Vault KV secrets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<VaultSecretOutput>,
    /// RefreshPeriod if specified, the operator will refresh the secret with the given frequency. This takes precedence over any vault secret lease duration and can be used to force a refresh.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshPeriod")]
    pub refresh_period: Option<String>,
    /// RefreshThreshold if specified, will instruct the operator to refresh when a percentage of the lease duration is met when there is no RefreshPeriod specified. This is particularly useful for controlling when dynamic secrets should be refreshed before the lease duration is exceeded. The default is 90, meaning the secret would refresh after 90% of the time has passed from the vault secret's lease duration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshThreshold")]
    pub refresh_threshold: Option<i64>,
    /// VaultSecretDefinitions are the secrets in Vault.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSecretDefinitions")]
    pub vault_secret_definitions: Option<Vec<VaultSecretVaultSecretDefinitions>>,
}

/// TemplatizedK8sSecret is the formatted K8s Secret created by templating from the Vault KV secrets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretOutput {
    /// Annotations are annotations to add to the final K8s Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels are labels to add to the final K8s Secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name is the K8s Secret name to output to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// StringData is the K8s Secret stringData and allows specifying non-binary secret data in string form with go templating support to transform the Vault KV secrets into a formatted K8s Secret. The Sprig template library and Helm functions (like toYaml) are supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stringData")]
    pub string_data: Option<BTreeMap<String, String>>,
    /// Type is the K8s Secret type to output to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitions {
    /// Authentication is the kube auth configuraiton to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<VaultSecretVaultSecretDefinitionsAuthentication>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<VaultSecretVaultSecretDefinitionsConnection>,
    /// Name is an arbitrary, but unique, name for this KV Vault secret and referenced when templating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path is the path of the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// RequestPayload for POST type of requests, this field contains the payload of the request. Not used for GET requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestPayload")]
    pub request_payload: Option<BTreeMap<String, String>>,
    /// RequestType the type of request needed to retrieve a secret. Normally a GET, but some secret engnes require a POST.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestType")]
    pub request_type: Option<VaultSecretVaultSecretDefinitionsRequestType>,
}

/// Authentication is the kube auth configuraiton to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitionsAuthentication {
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
    pub service_account: Option<VaultSecretVaultSecretDefinitionsAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitionsAuthenticationServiceAccount {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitionsConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<VaultSecretVaultSecretDefinitionsConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitionsConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<VaultSecretVaultSecretDefinitionsConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretVaultSecretDefinitionsConnectionTLsConfigTlsSecret {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultSecretVaultSecretDefinitionsRequestType {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}

/// VaultSecretStatus defines the observed state of VaultSecret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastVaultSecretUpdate the last time when this secret was updated from Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastVaultSecretUpdate")]
    pub last_vault_secret_update: Option<String>,
    /// NextVaultSecretUpdate the next time when this secret will be synced with Vault. If nil, it will not be refreshed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nextVaultSecretUpdate")]
    pub next_vault_secret_update: Option<String>,
    /// VaultSecretDefinitionsStatus information used to determine if the secret should be rereconciled
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultSecretDefinitionsStatus")]
    pub vault_secret_definitions_status: Option<Vec<VaultSecretStatusVaultSecretDefinitionsStatus>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VaultSecretStatusVaultSecretDefinitionsStatus {
    /// LeaseDuration is the time until the secret should be read in again, thus recreating the k8s Secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<i64>,
    /// LeaseID is the id of a lease, this denotes the secret is dynamic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    /// Name is an arbitrary, but unique, name for this KV Vault secret and referenced when templating.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Renewable informs if the lease is renewable for the dynamic secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewable: Option<bool>,
}

