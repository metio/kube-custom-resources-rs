// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kedacore/keda/keda.sh/v1alpha1/clustertriggerauthentications.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TriggerAuthenticationSpec defines the various ways to authenticate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keda.sh", version = "v1alpha1", kind = "ClusterTriggerAuthentication", plural = "clustertriggerauthentications")]
#[kube(status = "ClusterTriggerAuthenticationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterTriggerAuthenticationSpec {
    /// AwsSecretManager is used to authenticate using AwsSecretManager
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsSecretManager")]
    pub aws_secret_manager: Option<ClusterTriggerAuthenticationAwsSecretManager>,
    /// AzureKeyVault is used to authenticate using Azure Key Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "azureKeyVault")]
    pub azure_key_vault: Option<ClusterTriggerAuthenticationAzureKeyVault>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundServiceAccountToken")]
    pub bound_service_account_token: Option<Vec<ClusterTriggerAuthenticationBoundServiceAccountToken>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapTargetRef")]
    pub config_map_target_ref: Option<Vec<ClusterTriggerAuthenticationConfigMapTargetRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<ClusterTriggerAuthenticationEnv>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcpSecretManager")]
    pub gcp_secret_manager: Option<ClusterTriggerAuthenticationGcpSecretManager>,
    /// HashiCorpVault is used to authenticate using Hashicorp Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hashiCorpVault")]
    pub hashi_corp_vault: Option<ClusterTriggerAuthenticationHashiCorpVault>,
    /// AuthPodIdentity allows users to select the platform native identity
    /// mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<ClusterTriggerAuthenticationPodIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretTargetRef")]
    pub secret_target_ref: Option<Vec<ClusterTriggerAuthenticationSecretTargetRef>>,
}

/// AwsSecretManager is used to authenticate using AwsSecretManager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManager {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ClusterTriggerAuthenticationAwsSecretManagerCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity
    /// mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<ClusterTriggerAuthenticationAwsSecretManagerPodIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub secrets: Vec<ClusterTriggerAuthenticationAwsSecretManagerSecrets>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentials {
    #[serde(rename = "accessKey")]
    pub access_key: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKey,
    #[serde(rename = "accessSecretKey")]
    pub access_secret_key: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKey,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessToken")]
    pub access_token: Option<ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKey {
    #[serde(rename = "valueFrom")]
    pub value_from: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKey {
    #[serde(rename = "valueFrom")]
    pub value_from: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessToken {
    #[serde(rename = "valueFrom")]
    pub value_from: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerPodIdentity {
    /// Set identityAuthorityHost to override the default Azure authority host. If this is set, then the IdentityTenantID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityAuthorityHost")]
    pub identity_authority_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<ClusterTriggerAuthenticationAwsSecretManagerPodIdentityIdentityOwner>,
    /// Set identityTenantId to override the default Azure tenant id. If this is set, then the IdentityID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityTenantId")]
    pub identity_tenant_id: Option<String>,
    /// PodIdentityProvider contains the list of providers
    pub provider: ClusterTriggerAuthenticationAwsSecretManagerPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationAwsSecretManagerPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationAwsSecretManagerPodIdentityProvider {
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAwsSecretManagerSecrets {
    pub name: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKey")]
    pub secret_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionId")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionStage")]
    pub version_stage: Option<String>,
}

/// AzureKeyVault is used to authenticate using Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<ClusterTriggerAuthenticationAzureKeyVaultCloud>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ClusterTriggerAuthenticationAzureKeyVaultCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity
    /// mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<ClusterTriggerAuthenticationAzureKeyVaultPodIdentity>,
    pub secrets: Vec<ClusterTriggerAuthenticationAzureKeyVaultSecrets>,
    #[serde(rename = "vaultUri")]
    pub vault_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultCloud {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDirectoryEndpoint")]
    pub active_directory_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyVaultResourceURL")]
    pub key_vault_resource_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultCredentials {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecret,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecret {
    #[serde(rename = "valueFrom")]
    pub value_from: ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultPodIdentity {
    /// Set identityAuthorityHost to override the default Azure authority host. If this is set, then the IdentityTenantID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityAuthorityHost")]
    pub identity_authority_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<ClusterTriggerAuthenticationAzureKeyVaultPodIdentityIdentityOwner>,
    /// Set identityTenantId to override the default Azure tenant id. If this is set, then the IdentityID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityTenantId")]
    pub identity_tenant_id: Option<String>,
    /// PodIdentityProvider contains the list of providers
    pub provider: ClusterTriggerAuthenticationAzureKeyVaultPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationAzureKeyVaultPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationAzureKeyVaultPodIdentityProvider {
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationAzureKeyVaultSecrets {
    pub name: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationBoundServiceAccountToken {
    pub parameter: String,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: String,
}

/// AuthConfigMapTargetRef is used to authenticate using a reference to a config map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationConfigMapTargetRef {
    pub key: String,
    pub name: String,
    pub parameter: String,
}

/// AuthEnvironment is used to authenticate using environment variables
/// in the destination ScaleTarget spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationEnv {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    pub name: String,
    pub parameter: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManager {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ClusterTriggerAuthenticationGcpSecretManagerCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity
    /// mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<ClusterTriggerAuthenticationGcpSecretManagerPodIdentity>,
    pub secrets: Vec<ClusterTriggerAuthenticationGcpSecretManagerSecrets>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerCredentials {
    #[serde(rename = "clientSecret")]
    pub client_secret: ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecret,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecret {
    #[serde(rename = "valueFrom")]
    pub value_from: ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerPodIdentity {
    /// Set identityAuthorityHost to override the default Azure authority host. If this is set, then the IdentityTenantID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityAuthorityHost")]
    pub identity_authority_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<ClusterTriggerAuthenticationGcpSecretManagerPodIdentityIdentityOwner>,
    /// Set identityTenantId to override the default Azure tenant id. If this is set, then the IdentityID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityTenantId")]
    pub identity_tenant_id: Option<String>,
    /// PodIdentityProvider contains the list of providers
    pub provider: ClusterTriggerAuthenticationGcpSecretManagerPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationGcpSecretManagerPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationGcpSecretManagerPodIdentityProvider {
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationGcpSecretManagerSecrets {
    pub id: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// HashiCorpVault is used to authenticate using Hashicorp Vault
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationHashiCorpVault {
    pub address: String,
    /// VaultAuthentication contains the list of Hashicorp Vault authentication methods
    pub authentication: String,
    /// Credential defines the Hashicorp Vault credentials depending on the authentication method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<ClusterTriggerAuthenticationHashiCorpVaultCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub secrets: Vec<ClusterTriggerAuthenticationHashiCorpVaultSecrets>,
}

/// Credential defines the Hashicorp Vault credentials depending on the authentication method
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationHashiCorpVaultCredential {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// VaultSecret defines the mapping between the path of the secret in Vault to the parameter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationHashiCorpVaultSecrets {
    pub key: String,
    pub parameter: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pkiData")]
    pub pki_data: Option<ClusterTriggerAuthenticationHashiCorpVaultSecretsPkiData>,
    /// VaultSecretType defines the type of vault secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationHashiCorpVaultSecretsPkiData {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "altNames")]
    pub alt_names: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commonName")]
    pub common_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipSans")]
    pub ip_sans: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "otherSans")]
    pub other_sans: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uriSans")]
    pub uri_sans: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterTriggerAuthenticationPodIdentity {
    /// Set identityAuthorityHost to override the default Azure authority host. If this is set, then the IdentityTenantID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityAuthorityHost")]
    pub identity_authority_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<ClusterTriggerAuthenticationPodIdentityIdentityOwner>,
    /// Set identityTenantId to override the default Azure tenant id. If this is set, then the IdentityID must also be set
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityTenantId")]
    pub identity_tenant_id: Option<String>,
    /// PodIdentityProvider contains the list of providers
    pub provider: ClusterTriggerAuthenticationPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity
/// mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterTriggerAuthenticationPodIdentityProvider {
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "none")]
    None,
}

/// AuthSecretTargetRef is used to authenticate using a reference to a secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationSecretTargetRef {
    pub key: String,
    pub name: String,
    pub parameter: String,
}

/// TriggerAuthenticationStatus defines the observed state of TriggerAuthentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterTriggerAuthenticationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaledjobs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaledobjects: Option<String>,
}

