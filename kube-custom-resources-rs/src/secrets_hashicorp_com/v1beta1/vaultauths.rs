// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/vaultauths.yaml --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// VaultAuthSpec defines the desired state of VaultAuth
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "VaultAuth", plural = "vaultauths")]
#[kube(namespaced)]
#[kube(status = "VaultAuthStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct VaultAuthSpec {
    /// AllowedNamespaces Kubernetes Namespaces which are allow-listed for use with this AuthMethod.
    /// This field allows administrators to customize which Kubernetes namespaces are authorized to
    /// use with this AuthMethod. While Vault will still enforce its own rules, this has the added
    /// configurability of restricting which VaultAuthMethods can be used by which namespaces.
    /// Accepted values:
    /// []{"*"} - wildcard, all namespaces.
    /// []{"a", "b"} - list of namespaces.
    /// unset - disallow all namespaces except the Operator's the VaultAuthMethod's namespace, this
    /// is the default behavior.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedNamespaces")]
    pub allowed_namespaces: Option<Vec<String>>,
    /// AppRole specific auth configuration, requires that the Method be set to `appRole`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appRole")]
    pub app_role: Option<VaultAuthAppRole>,
    /// AWS specific auth configuration, requires that Method be set to `aws`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<VaultAuthAws>,
    /// GCP specific auth configuration, requires that Method be set to `gcp`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcp: Option<VaultAuthGcp>,
    /// Headers to be included in all Vault requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    /// JWT specific auth configuration, requires that the Method be set to `jwt`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt: Option<VaultAuthJwt>,
    /// Kubernetes specific auth configuration, requires that the Method be set to `kubernetes`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<VaultAuthKubernetes>,
    /// Method to use when authenticating to Vault.
    pub method: VaultAuthMethod,
    /// Mount to use when authenticating to auth method.
    pub mount: String,
    /// Namespace to auth to in Vault
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Params to use when authenticating to Vault
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// StorageEncryption provides the necessary configuration to encrypt the client storage cache.
    /// This should only be configured when client cache persistence with encryption is enabled.
    /// This is done by passing setting the manager's commandline argument
    /// --client-cache-persistence-model=direct-encrypted. Typically, there should only ever
    /// be one VaultAuth configured with StorageEncryption in the Cluster, and it should have
    /// the label: cacheStorageEncryption=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageEncryption")]
    pub storage_encryption: Option<VaultAuthStorageEncryption>,
    /// VaultConnectionRef to the VaultConnection resource, can be prefixed with a namespace,
    /// eg: `namespaceA/vaultConnectionRefB`. If no namespace prefix is provided it will default to
    /// namespace of the VaultConnection CR. If no value is specified for VaultConnectionRef the
    /// Operator will default to the `default` VaultConnection, configured in the operator's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultConnectionRef")]
    pub vault_connection_ref: Option<String>,
}

/// AppRole specific auth configuration, requires that the Method be set to `appRole`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthAppRole {
    /// RoleID of the AppRole Role to use for authenticating to Vault.
    #[serde(rename = "roleId")]
    pub role_id: String,
    /// SecretRef is the name of a Kubernetes secret in the consumer's (VDS/VSS/PKI) namespace which
    /// provides the AppRole Role's SecretID. The secret must have a key named `id` which holds the
    /// AppRole Role's secretID.
    #[serde(rename = "secretRef")]
    pub secret_ref: String,
}

/// AWS specific auth configuration, requires that Method be set to `aws`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthAws {
    /// The Vault header value to include in the STS signing request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "headerValue")]
    pub header_value: Option<String>,
    /// The IAM endpoint to use; if not set will use the default
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iamEndpoint")]
    pub iam_endpoint: Option<String>,
    /// IRSAServiceAccount name to use with IAM Roles for Service Accounts
    /// (IRSA), and should be annotated with "eks.amazonaws.com/role-arn". This
    /// ServiceAccount will be checked for other EKS annotations:
    /// eks.amazonaws.com/audience and eks.amazonaws.com/token-expiration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "irsaServiceAccount")]
    pub irsa_service_account: Option<String>,
    /// AWS Region to use for signing the authentication request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Vault role to use for authenticating
    pub role: String,
    /// SecretRef is the name of a Kubernetes Secret in the consumer's (VDS/VSS/PKI) namespace
    /// which holds credentials for AWS. Expected keys include `access_key_id`, `secret_access_key`,
    /// `session_token`
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<String>,
    /// The role session name to use when creating a webidentity provider
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionName")]
    pub session_name: Option<String>,
    /// The STS endpoint to use; if not set will use the default
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stsEndpoint")]
    pub sts_endpoint: Option<String>,
}

/// GCP specific auth configuration, requires that Method be set to `gcp`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthGcp {
    /// GKE cluster name. Defaults to the cluster-name returned from the operator
    /// pod's local metadata server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    /// GCP project ID. Defaults to the project-id returned from the operator
    /// pod's local metadata server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectID")]
    pub project_id: Option<String>,
    /// GCP Region of the GKE cluster's identity provider. Defaults to the region
    /// returned from the operator pod's local metadata server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Vault role to use for authenticating
    pub role: String,
    /// WorkloadIdentityServiceAccount is the name of a Kubernetes service
    /// account (in the same Kubernetes namespace as the Vault*Secret referencing
    /// this resource) which has been configured for workload identity in GKE.
    /// Should be annotated with "iam.gke.io/gcp-service-account".
    #[serde(rename = "workloadIdentityServiceAccount")]
    pub workload_identity_service_account: String,
}

/// JWT specific auth configuration, requires that the Method be set to `jwt`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthJwt {
    /// TokenAudiences to include in the ServiceAccount token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// Role to use for authenticating to Vault.
    pub role: String,
    /// SecretRef is the name of a Kubernetes secret in the consumer's (VDS/VSS/PKI) namespace which
    /// provides the JWT token to authenticate to Vault's JWT authentication backend. The secret must
    /// have a key named `jwt` which holds the JWT token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<String>,
    /// ServiceAccount to use when creating a ServiceAccount token to authenticate to Vault's
    /// JWT authentication backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    /// TokenExpirationSeconds to set the ServiceAccount token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenExpirationSeconds")]
    pub token_expiration_seconds: Option<i64>,
}

/// Kubernetes specific auth configuration, requires that the Method be set to `kubernetes`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthKubernetes {
    /// TokenAudiences to include in the ServiceAccount token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audiences: Option<Vec<String>>,
    /// Role to use for authenticating to Vault.
    pub role: String,
    /// ServiceAccount to use when authenticating to Vault's
    /// authentication backend. This must reside in the consuming secret's (VDS/VSS/PKI) namespace.
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    /// TokenExpirationSeconds to set the ServiceAccount token.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenExpirationSeconds")]
    pub token_expiration_seconds: Option<i64>,
}

/// VaultAuthSpec defines the desired state of VaultAuth
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultAuthMethod {
    #[serde(rename = "kubernetes")]
    Kubernetes,
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "appRole")]
    AppRole,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "gcp")]
    Gcp,
}

/// StorageEncryption provides the necessary configuration to encrypt the client storage cache.
/// This should only be configured when client cache persistence with encryption is enabled.
/// This is done by passing setting the manager's commandline argument
/// --client-cache-persistence-model=direct-encrypted. Typically, there should only ever
/// be one VaultAuth configured with StorageEncryption in the Cluster, and it should have
/// the label: cacheStorageEncryption=true
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthStorageEncryption {
    /// KeyName to use for encrypt/decrypt operations via Vault Transit.
    #[serde(rename = "keyName")]
    pub key_name: String,
    /// Mount path of the Transit engine in Vault.
    pub mount: String,
}

/// VaultAuthStatus defines the observed state of VaultAuth
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultAuthStatus {
    pub error: String,
    /// Valid auth mechanism.
    pub valid: bool,
}

