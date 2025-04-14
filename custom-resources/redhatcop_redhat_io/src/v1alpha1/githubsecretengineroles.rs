// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/githubsecretengineroles.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// GitHubSecretEngineRoleSpec defines the desired state of GitHubSecretEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "GitHubSecretEngineRole", plural = "githubsecretengineroles")]
#[kube(namespaced)]
#[kube(status = "GitHubSecretEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GitHubSecretEngineRoleSpec {
    /// Authentication is the kube auth configuration to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<GitHubSecretEngineRoleAuthentication>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<GitHubSecretEngineRoleConnection>,
    /// InstallationID the ID of the app installation. Note the Installation ID from the URL of this page (usually: https://github.com/settings/installations/<installation id>) if you wish to configure using the installation ID directly. Only one of installationID or organizationName is required. If both are provided, installationID takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "installationID")]
    pub installation_id: Option<i64>,
    /// The name of the obejct created in Vault. If this is specified it takes precedence over {metatada.name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OrganizationName the name of the organization with the GitHub App installation. Only one of installationID or organizationName is required. If both are provided, installationID takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "organizationName")]
    pub organization_name: Option<String>,
    /// Path at which to create the role. The final path in Vault will be {[spec.authentication.namespace]}/{spec.path}/permissionset/{metadata.name}. The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Permissions a key value map of permission names to their access type (read or write). See [GitHub’s documentation](https://developer.github.com/v3/apps/permissions) on permission names and access types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<BTreeMap<String, String>>,
    /// Repositories a list of the names of the repositories within the organisation that the installation token can access
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<String>>,
    /// Repositories a list of the IDs of the repositories that the installation token can access. See [this StackOverflow](https://stackoverflow.com/a/47223479) post for the quickest way to find a repository ID
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "repositoriesIDs")]
    pub repositories_i_ds: Option<Vec<String>>,
}

/// Authentication is the kube auth configuration to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleAuthentication {
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
    pub service_account: Option<GitHubSecretEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleAuthenticationServiceAccount {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<GitHubSecretEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<GitHubSecretEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// GitHubSecretEngineRoleStatus defines the observed state of GitHubSecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GitHubSecretEngineRoleStatus {
    /// Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

