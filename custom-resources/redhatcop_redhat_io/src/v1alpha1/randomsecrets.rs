// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/randomsecrets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RandomSecretSpec defines the desired state of RandomSecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "RandomSecret", plural = "randomsecrets")]
#[kube(namespaced)]
#[kube(status = "RandomSecretStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RandomSecretSpec {
    /// Authentication is the kube auth configuration to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<RandomSecretAuthentication>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<RandomSecretConnection>,
    /// IsKVSecretsEngineV2 indicates if the KV Secrets engine is V2 or not. Default is false to indicate the payload to send is for KV Secret Engine V1.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isKVSecretsEngineV2")]
    pub is_kv_secrets_engine_v2: Option<bool>,
    /// The name of the obejct created in Vault. If this is specified it takes precedence over {metatada.name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path at which to create the secret. The final path in Vault will be {[spec.authentication.namespace]}/{spec.path}/{metadata.name}. If IsKVSecretsEngineV2 is false, the authentication role must have the following capabilities = [ "create", "update", "delete"] on the {[spec.authentication.namespace]}/{spec.path}/{metadata.name} path. If IsKVSecretsEngineV2 is true, the authentication role must have the following capabilities = [ "create", "update"] on the {[spec.authentication.namespace]}/{spec.path}/data/{metadata.name} path and capabilities = [ "delete"] on the {[spec.authentication.namespace]}/{spec.path}/metadata/{metadata.name} path. Additionally, if IsKVSecretsEngineV2 is true, it is acceptable for this value to have a suffix of "/data" or not. This suffix is no longer needed but still supported for backwards compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// RefreshPeriod if specified, the operator will refresh the secret with the given frequency. This will also set the ttl of the secret which provides a hint for how often consumers should check back for a new value when reading the secret's lease_duration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshPeriod")]
    pub refresh_period: Option<String>,
    /// SecretFormat specifies a map of key and password policies used to generate random values
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretFormat")]
    pub secret_format: Option<RandomSecretSecretFormat>,
    /// SecretKey is the key to be used for this secret when stored in Vault kv
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKey")]
    pub secret_key: Option<String>,
}

/// Authentication is the kube auth configuration to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretAuthentication {
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
    pub service_account: Option<RandomSecretAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretAuthenticationServiceAccount {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<RandomSecretConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<RandomSecretConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretConnectionTLsConfigTlsSecret {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SecretFormat specifies a map of key and password policies used to generate random values
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretSecretFormat {
    /// InlinePasswordPolicy is an inline password policy specified using Vault password policy syntax (https://www.vaultproject.io/docs/concepts/password-policies#password-policy-syntax) Only one of PasswordPolicyName or InlinePasswordPolicy can be specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inlinePasswordPolicy")]
    pub inline_password_policy: Option<String>,
    /// PasswordPolicyName a ref to a password policy defined in Vault. Notice that in order to use this, the Vault role you use needs the following capabilities = ["read"] on /sys/policy/password. Only one of PasswordPolicyName or InlinePasswordPolicy can be specified
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passwordPolicyName")]
    pub password_policy_name: Option<String>,
}

/// RandomSecretStatus defines the observed state of RandomSecret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RandomSecretStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastVaultSecretUpdate last time when this secret was updated in Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastVaultSecretUpdate")]
    pub last_vault_secret_update: Option<String>,
}

