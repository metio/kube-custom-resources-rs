// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/vaultstaticsecrets.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VaultStaticSecretSpec defines the desired state of VaultStaticSecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "VaultStaticSecret", plural = "vaultstaticsecrets")]
#[kube(namespaced)]
#[kube(status = "VaultStaticSecretStatus")]
#[kube(schema = "disabled")]
pub struct VaultStaticSecretSpec {
    /// Destination provides configuration necessary for syncing the Vault secret to Kubernetes.
    pub destination: VaultStaticSecretDestination,
    /// HMACSecretData determines whether the Operator computes the HMAC of the Secret's data. The MAC value will be stored in the resource's Status.SecretMac field, and will be used for drift detection and during incoming Vault secret comparison. Enabling this feature is recommended to ensure that Secret's data stays consistent with Vault.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hmacSecretData")]
    pub hmac_secret_data: Option<bool>,
    /// Mount for the secret in Vault
    pub mount: String,
    /// Namespace to get the secret from in Vault
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Path of the secret in Vault, corresponds to the `path` parameter for, kv-v1: https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v1#read-secret kv-v2: https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#read-secret-version
    pub path: String,
    /// RefreshAfter a period of time, in duration notation e.g. 30s, 1m, 24h
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshAfter")]
    pub refresh_after: Option<String>,
    /// RolloutRestartTargets should be configured whenever the application(s) consuming the Vault secret does not support dynamically reloading a rotated secret. In that case one, or more RolloutRestartTarget(s) can be configured here. The Operator will trigger a "rollout-restart" for each target whenever the Vault secret changes between reconciliation events. All configured targets wil be ignored if HMACSecretData is set to false. See RolloutRestartTarget for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutRestartTargets")]
    pub rollout_restart_targets: Option<Vec<VaultStaticSecretRolloutRestartTargets>>,
    /// Type of the Vault static secret
    #[serde(rename = "type")]
    pub r#type: VaultStaticSecretType,
    /// VaultAuthRef to the VaultAuth resource, can be prefixed with a namespace, eg: `namespaceA/vaultAuthRefB`. If no namespace prefix is provided it will default to namespace of the VaultAuth CR. If no value is specified for VaultAuthRef the Operator will default to the `default` VaultAuth, configured in the operator's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultAuthRef")]
    pub vault_auth_ref: Option<String>,
    /// Version of the secret to fetch. Only valid for type kv-v2. Corresponds to version query parameter: https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2#version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// Destination provides configuration necessary for syncing the Vault secret to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultStaticSecretDestination {
    /// Annotations to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Create the destination Secret. If the Secret already exists this should be set to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    /// Labels to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the Secret
    pub name: String,
    /// Type of Kubernetes Secret. Requires Create to be set to true. Defaults to Opaque.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultStaticSecretRolloutRestartTargets {
    pub kind: VaultStaticSecretRolloutRestartTargetsKind,
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultStaticSecretRolloutRestartTargetsKind {
    Deployment,
    DaemonSet,
    StatefulSet,
}

/// VaultStaticSecretSpec defines the desired state of VaultStaticSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultStaticSecretType {
    #[serde(rename = "kv-v1")]
    KvV1,
    #[serde(rename = "kv-v2")]
    KvV2,
}

/// VaultStaticSecretStatus defines the observed state of VaultStaticSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultStaticSecretStatus {
    /// LastGeneration is the Generation of the last reconciled resource.
    #[serde(rename = "lastGeneration")]
    pub last_generation: i64,
    /// SecretMAC used when deciding whether new Vault secret data should be synced. 
    ///  The controller will compare the "new" Vault secret data to this value using HMAC, if they are different, then the data will be synced to the Destination. 
    ///  The SecretMac is also used to detect drift in the Destination Secret's Data. If drift is detected the data will be synced to the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretMAC")]
    pub secret_mac: Option<String>,
}

