// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kedacore/keda/keda.sh/v1alpha1/triggerauthentications.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// TriggerAuthenticationSpec defines the various ways to authenticate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "keda.sh", version = "v1alpha1", kind = "TriggerAuthentication", plural = "triggerauthentications")]
#[kube(namespaced)]
#[kube(status = "TriggerAuthenticationStatus")]
#[kube(schema = "disabled")]
pub struct TriggerAuthenticationSpec {
    /// AwsSecretManager is used to authenticate using AwsSecretManager
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "awsSecretManager")]
    pub aws_secret_manager: Option<TriggerAuthenticationAwsSecretManager>,
    /// AzureKeyVault is used to authenticate using Azure Key Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "azureKeyVault")]
    pub azure_key_vault: Option<TriggerAuthenticationAzureKeyVault>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapTargetRef")]
    pub config_map_target_ref: Option<Vec<TriggerAuthenticationConfigMapTargetRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<TriggerAuthenticationEnv>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gcpSecretManager")]
    pub gcp_secret_manager: Option<TriggerAuthenticationGcpSecretManager>,
    /// HashiCorpVault is used to authenticate using Hashicorp Vault
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hashiCorpVault")]
    pub hashi_corp_vault: Option<TriggerAuthenticationHashiCorpVault>,
    /// AuthPodIdentity allows users to select the platform native identity mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<TriggerAuthenticationPodIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretTargetRef")]
    pub secret_target_ref: Option<Vec<TriggerAuthenticationSecretTargetRef>>,
}

/// AwsSecretManager is used to authenticate using AwsSecretManager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManager {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TriggerAuthenticationAwsSecretManagerCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<TriggerAuthenticationAwsSecretManagerPodIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub secrets: Vec<TriggerAuthenticationAwsSecretManagerSecrets>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentials {
    #[serde(rename = "accessKey")]
    pub access_key: TriggerAuthenticationAwsSecretManagerCredentialsAccessKey,
    #[serde(rename = "accessSecretKey")]
    pub access_secret_key: TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKey,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessToken")]
    pub access_token: Option<TriggerAuthenticationAwsSecretManagerCredentialsAccessToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessKey {
    #[serde(rename = "valueFrom")]
    pub value_from: TriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: TriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessKeyValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKey {
    #[serde(rename = "valueFrom")]
    pub value_from: TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessSecretKeyValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessToken {
    #[serde(rename = "valueFrom")]
    pub value_from: TriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: TriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerCredentialsAccessTokenValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerPodIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<TriggerAuthenticationAwsSecretManagerPodIdentityIdentityOwner>,
    /// PodIdentityProvider contains the list of providers
    pub provider: TriggerAuthenticationAwsSecretManagerPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationAwsSecretManagerPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationAwsSecretManagerPodIdentityProvider {
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "aws-kiam")]
    AwsKiam,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAwsSecretManagerSecrets {
    pub name: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionId")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "versionStage")]
    pub version_stage: Option<String>,
}

/// AzureKeyVault is used to authenticate using Azure Key Vault
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<TriggerAuthenticationAzureKeyVaultCloud>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TriggerAuthenticationAzureKeyVaultCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<TriggerAuthenticationAzureKeyVaultPodIdentity>,
    pub secrets: Vec<TriggerAuthenticationAzureKeyVaultSecrets>,
    #[serde(rename = "vaultUri")]
    pub vault_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultCloud {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDirectoryEndpoint")]
    pub active_directory_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyVaultResourceURL")]
    pub key_vault_resource_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultCredentials {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: TriggerAuthenticationAzureKeyVaultCredentialsClientSecret,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultCredentialsClientSecret {
    #[serde(rename = "valueFrom")]
    pub value_from: TriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: TriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultCredentialsClientSecretValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultPodIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<TriggerAuthenticationAzureKeyVaultPodIdentityIdentityOwner>,
    /// PodIdentityProvider contains the list of providers
    pub provider: TriggerAuthenticationAzureKeyVaultPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationAzureKeyVaultPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationAzureKeyVaultPodIdentityProvider {
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "aws-kiam")]
    AwsKiam,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationAzureKeyVaultSecrets {
    pub name: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// AuthConfigMapTargetRef is used to authenticate using a reference to a config map
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationConfigMapTargetRef {
    pub key: String,
    pub name: String,
    pub parameter: String,
}

/// AuthEnvironment is used to authenticate using environment variables in the destination ScaleTarget spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationEnv {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    pub name: String,
    pub parameter: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManager {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TriggerAuthenticationGcpSecretManagerCredentials>,
    /// AuthPodIdentity allows users to select the platform native identity mechanism
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podIdentity")]
    pub pod_identity: Option<TriggerAuthenticationGcpSecretManagerPodIdentity>,
    pub secrets: Vec<TriggerAuthenticationGcpSecretManagerSecrets>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerCredentials {
    #[serde(rename = "clientSecret")]
    pub client_secret: TriggerAuthenticationGcpSecretManagerCredentialsClientSecret,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerCredentialsClientSecret {
    #[serde(rename = "valueFrom")]
    pub value_from: TriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFrom,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFrom {
    #[serde(rename = "secretKeyRef")]
    pub secret_key_ref: TriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFromSecretKeyRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerCredentialsClientSecretValueFromSecretKeyRef {
    pub key: String,
    pub name: String,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerPodIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<TriggerAuthenticationGcpSecretManagerPodIdentityIdentityOwner>,
    /// PodIdentityProvider contains the list of providers
    pub provider: TriggerAuthenticationGcpSecretManagerPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationGcpSecretManagerPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationGcpSecretManagerPodIdentityProvider {
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "aws-kiam")]
    AwsKiam,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationGcpSecretManagerSecrets {
    pub id: String,
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// HashiCorpVault is used to authenticate using Hashicorp Vault
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationHashiCorpVault {
    pub address: String,
    /// VaultAuthentication contains the list of Hashicorp Vault authentication methods
    pub authentication: String,
    /// Credential defines the Hashicorp Vault credentials depending on the authentication method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<TriggerAuthenticationHashiCorpVaultCredential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub secrets: Vec<TriggerAuthenticationHashiCorpVaultSecrets>,
}

/// Credential defines the Hashicorp Vault credentials depending on the authentication method
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationHashiCorpVaultCredential {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// VaultSecret defines the mapping between the path of the secret in Vault to the parameter
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationHashiCorpVaultSecrets {
    pub key: String,
    pub parameter: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pkiData")]
    pub pki_data: Option<TriggerAuthenticationHashiCorpVaultSecretsPkiData>,
    /// VaultSecretType defines the type of vault secret
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationHashiCorpVaultSecretsPkiData {
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

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationPodIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityId")]
    pub identity_id: Option<String>,
    /// IdentityOwner configures which identity has to be used during auto discovery, keda or the scaled workload. Mutually exclusive with roleArn
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityOwner")]
    pub identity_owner: Option<TriggerAuthenticationPodIdentityIdentityOwner>,
    /// PodIdentityProvider contains the list of providers
    pub provider: TriggerAuthenticationPodIdentityProvider,
    /// RoleArn sets the AWS RoleArn to be used. Mutually exclusive with IdentityOwner
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleArn")]
    pub role_arn: Option<String>,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationPodIdentityIdentityOwner {
    #[serde(rename = "keda")]
    Keda,
    #[serde(rename = "workload")]
    Workload,
}

/// AuthPodIdentity allows users to select the platform native identity mechanism
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TriggerAuthenticationPodIdentityProvider {
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "azure-workload")]
    AzureWorkload,
    #[serde(rename = "gcp")]
    Gcp,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "aws-eks")]
    AwsEks,
    #[serde(rename = "aws-kiam")]
    AwsKiam,
}

/// AuthSecretTargetRef is used to authenticate using a reference to a secret
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationSecretTargetRef {
    pub key: String,
    pub name: String,
    pub parameter: String,
}

/// TriggerAuthenticationStatus defines the observed state of TriggerAuthentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TriggerAuthenticationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaledjobs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaledobjects: Option<String>,
}

