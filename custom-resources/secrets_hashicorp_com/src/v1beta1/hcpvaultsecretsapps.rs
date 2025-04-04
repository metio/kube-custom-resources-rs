// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/hcpvaultsecretsapps.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// HCPVaultSecretsAppSpec defines the desired state of HCPVaultSecretsApp
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "HCPVaultSecretsApp", plural = "hcpvaultsecretsapps")]
#[kube(namespaced)]
#[kube(status = "HCPVaultSecretsAppStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HCPVaultSecretsAppSpec {
    /// AppName of the Vault Secrets Application that is to be synced.
    #[serde(rename = "appName")]
    pub app_name: String,
    /// Destination provides configuration necessary for syncing the HCP Vault
    /// Application secrets to Kubernetes.
    pub destination: HCPVaultSecretsAppDestination,
    /// HCPAuthRef to the HCPAuth resource, can be prefixed with a namespace, eg:
    /// `namespaceA/vaultAuthRefB`. If no namespace prefix is provided it will default
    /// to the namespace of the HCPAuth CR. If no value is specified for HCPAuthRef the
    /// Operator will default to the `default` HCPAuth, configured in the operator's
    /// namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hcpAuthRef")]
    pub hcp_auth_ref: Option<String>,
    /// RefreshAfter a period of time, in duration notation e.g. 30s, 1m, 24h
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refreshAfter")]
    pub refresh_after: Option<String>,
    /// RolloutRestartTargets should be configured whenever the application(s)
    /// consuming the HCP Vault Secrets App does not support dynamically reloading a
    /// rotated secret. In that case one, or more RolloutRestartTarget(s) can be
    /// configured here. The Operator will trigger a "rollout-restart" for each target
    /// whenever the Vault secret changes between reconciliation events. See
    /// RolloutRestartTarget for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutRestartTargets")]
    pub rollout_restart_targets: Option<Vec<HCPVaultSecretsAppRolloutRestartTargets>>,
    /// SyncConfig configures sync behavior from HVS to VSO
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncConfig")]
    pub sync_config: Option<HCPVaultSecretsAppSyncConfig>,
}

/// Destination provides configuration necessary for syncing the HCP Vault
/// Application secrets to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppDestination {
    /// Annotations to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Create the destination Secret.
    /// If the Secret already exists this should be set to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<bool>,
    /// Labels to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the Secret
    pub name: String,
    /// Overwrite the destination Secret if it exists and Create is true. This is
    /// useful when migrating to VSO from a previous secret deployment strategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    /// Transformation provides configuration for transforming the secret data before
    /// it is stored in the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transformation: Option<HCPVaultSecretsAppDestinationTransformation>,
    /// Type of Kubernetes Secret. Requires Create to be set to true.
    /// Defaults to Opaque.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Transformation provides configuration for transforming the secret data before
/// it is stored in the Destination.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppDestinationTransformation {
    /// ExcludeRaw data from the destination Secret. Exclusion policy can be set
    /// globally by including 'exclude-raw` in the '--global-transformation-options'
    /// command line flag. If set, the command line flag always takes precedence over
    /// this configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeRaw")]
    pub exclude_raw: Option<bool>,
    /// Excludes contains regex patterns used to filter top-level source secret data
    /// fields for exclusion from the final K8s Secret data. These pattern filters are
    /// never applied to templated fields as defined in Templates. They are always
    /// applied before any inclusion patterns. To exclude all source secret data
    /// fields, you can configure the single pattern ".*".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    /// Includes contains regex patterns used to filter top-level source secret data
    /// fields for inclusion in the final K8s Secret data. These pattern filters are
    /// never applied to templated fields as defined in Templates. They are always
    /// applied last.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
    /// Templates maps a template name to its Template. Templates are always included
    /// in the rendered K8s Secret, and take precedence over templates defined in a
    /// SecretTransformation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<BTreeMap<String, HCPVaultSecretsAppDestinationTransformationTemplates>>,
    /// TransformationRefs contain references to template configuration from
    /// SecretTransformation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transformationRefs")]
    pub transformation_refs: Option<Vec<HCPVaultSecretsAppDestinationTransformationTransformationRefs>>,
}

/// Templates maps a template name to its Template. Templates are always included
/// in the rendered K8s Secret, and take precedence over templates defined in a
/// SecretTransformation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppDestinationTransformationTemplates {
    /// Name of the Template
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Text contains the Go text template format. The template
    /// references attributes from the data structure of the source secret.
    /// Refer to https://pkg.go.dev/text/template for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// TransformationRef contains the configuration for accessing templates from an
/// SecretTransformation resource. TransformationRefs can be shared across all
/// syncable secret custom resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppDestinationTransformationTransformationRefs {
    /// IgnoreExcludes controls whether to use the SecretTransformation's Excludes
    /// data key filters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreExcludes")]
    pub ignore_excludes: Option<bool>,
    /// IgnoreIncludes controls whether to use the SecretTransformation's Includes
    /// data key filters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreIncludes")]
    pub ignore_includes: Option<bool>,
    /// Name of the SecretTransformation resource.
    pub name: String,
    /// Namespace of the SecretTransformation resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// TemplateRefs map to a Template found in this TransformationRef. If empty, then
    /// all templates from the SecretTransformation will be rendered to the K8s Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateRefs")]
    pub template_refs: Option<Vec<HCPVaultSecretsAppDestinationTransformationTransformationRefsTemplateRefs>>,
}

/// TemplateRef points to templating text that is stored in a
/// SecretTransformation custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppDestinationTransformationTransformationRefsTemplateRefs {
    /// KeyOverride to the rendered template in the Destination secret. If Key is
    /// empty, then the Key from reference spec will be used. Set this to override the
    /// Key set from the reference spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyOverride")]
    pub key_override: Option<String>,
    /// Name of the Template in SecretTransformationSpec.Templates.
    /// the rendered secret data.
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a
/// rollout-restart of the supported resources upon Vault Secret rotation.
/// The rollout-restart is triggered by patching the target resource's
/// 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt'
/// with a timestamp value of when the trigger was executed.
/// E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z"
/// 
/// Supported resources: Deployment, DaemonSet, StatefulSet, argo.Rollout
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HCPVaultSecretsAppRolloutRestartTargets {
    /// Kind of the resource
    pub kind: HCPVaultSecretsAppRolloutRestartTargetsKind,
    /// Name of the resource
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a
/// rollout-restart of the supported resources upon Vault Secret rotation.
/// The rollout-restart is triggered by patching the target resource's
/// 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt'
/// with a timestamp value of when the trigger was executed.
/// E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z"
/// 
/// Supported resources: Deployment, DaemonSet, StatefulSet, argo.Rollout
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HCPVaultSecretsAppRolloutRestartTargetsKind {
    Deployment,
    DaemonSet,
    StatefulSet,
    #[serde(rename = "argo.Rollout")]
    ArgoRollout,
}

/// SyncConfig configures sync behavior from HVS to VSO
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppSyncConfig {
    /// Dynamic configures sync behavior for dynamic secrets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<HCPVaultSecretsAppSyncConfigDynamic>,
}

/// Dynamic configures sync behavior for dynamic secrets.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppSyncConfigDynamic {
    /// RenewalPercent is the percent out of 100 of a dynamic secret's TTL when
    /// new secrets are generated. Defaults to 67 percent plus up to 10% jitter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renewalPercent")]
    pub renewal_percent: Option<i64>,
}

/// HCPVaultSecretsAppStatus defines the observed state of HCPVaultSecretsApp
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppStatus {
    /// DynamicSecrets lists the last observed state of any dynamic secrets
    /// within the HCP Vault Secrets App
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dynamicSecrets")]
    pub dynamic_secrets: Option<Vec<HCPVaultSecretsAppStatusDynamicSecrets>>,
    /// LastGeneration is the Generation of the last reconciled resource.
    #[serde(rename = "lastGeneration")]
    pub last_generation: i64,
    /// SecretMAC used when deciding whether new Vault secret data should be synced.
    /// 
    /// The controller will compare the "new" HCP Vault Secrets App data to this value
    /// using HMAC, if they are different, then the data will be synced to the
    /// Destination.
    /// 
    /// The SecretMac is also used to detect drift in the Destination Secret's Data.
    /// If drift is detected the data will be synced to the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretMAC")]
    pub secret_mac: Option<String>,
}

/// HVSDynamicStatus defines the observed state of a dynamic secret within an HCP
/// Vault Secrets App
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HCPVaultSecretsAppStatusDynamicSecrets {
    /// CreatedAt is the timestamp string of when the dynamic secret was created
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createdAt")]
    pub created_at: Option<String>,
    /// ExpiresAt is the timestamp string of when the dynamic secret will expire
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expiresAt")]
    pub expires_at: Option<String>,
    /// Name of the dynamic secret
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// TTL is the time-to-live of the dynamic secret in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

