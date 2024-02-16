// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/hashicorp/vault-secrets-operator/secrets.hashicorp.com/v1beta1/vaultdynamicsecrets.yaml --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// VaultDynamicSecretSpec defines the desired state of VaultDynamicSecret
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "secrets.hashicorp.com", version = "v1beta1", kind = "VaultDynamicSecret", plural = "vaultdynamicsecrets")]
#[kube(namespaced)]
#[kube(status = "VaultDynamicSecretStatus")]
#[kube(schema = "disabled")]
pub struct VaultDynamicSecretSpec {
    /// AllowStaticCreds should be set when syncing credentials that are periodically rotated by the Vault server, rather than created upon request. These secrets are sometimes referred to as "static roles", or "static credentials", with a request path that contains "static-creds".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowStaticCreds")]
    pub allow_static_creds: Option<bool>,
    /// Destination provides configuration necessary for syncing the Vault secret to Kubernetes.
    pub destination: VaultDynamicSecretDestination,
    /// Mount path of the secret's engine in Vault.
    pub mount: String,
    /// Namespace where the secrets engine is mounted in Vault.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Params that can be passed when requesting credentials/secrets. When Params is set the configured RequestHTTPMethod will be ignored. See RequestHTTPMethod for more details. Please consult https://developer.hashicorp.com/vault/docs/secrets if you are uncertain about what 'params' should/can be set to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    /// Path in Vault to get the credentials for, and is relative to Mount. Please consult https://developer.hashicorp.com/vault/docs/secrets if you are uncertain about what 'path' should be set to.
    pub path: String,
    /// RenewalPercent is the percent out of 100 of the lease duration when the lease is renewed. Defaults to 67 percent plus jitter.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "renewalPercent")]
    pub renewal_percent: Option<i64>,
    /// RequestHTTPMethod to use when syncing Secrets from Vault. Setting a value here is not typically required. If left unset the Operator will make requests using the GET method. In the case where Params are specified the Operator will use the PUT method. Please consult https://developer.hashicorp.com/vault/docs/secrets if you are uncertain about what method to use. Of note, the Vault client treats PUT and POST as being equivalent. The underlying Vault client implementation will always use the PUT method.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHTTPMethod")]
    pub request_http_method: Option<VaultDynamicSecretRequestHttpMethod>,
    /// Revoke the existing lease on VDS resource deletion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke: Option<bool>,
    /// RolloutRestartTargets should be configured whenever the application(s) consuming the Vault secret does not support dynamically reloading a rotated secret. In that case one, or more RolloutRestartTarget(s) can be configured here. The Operator will trigger a "rollout-restart" for each target whenever the Vault secret changes between reconciliation events. See RolloutRestartTarget for more details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rolloutRestartTargets")]
    pub rollout_restart_targets: Option<Vec<VaultDynamicSecretRolloutRestartTargets>>,
    /// VaultAuthRef to the VaultAuth resource, can be prefixed with a namespace, eg: `namespaceA/vaultAuthRefB`. If no namespace prefix is provided it will default to namespace of the VaultAuth CR. If no value is specified for VaultAuthRef the Operator will default to the `default` VaultAuth, configured in the operator's namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vaultAuthRef")]
    pub vault_auth_ref: Option<String>,
}

/// Destination provides configuration necessary for syncing the Vault secret to Kubernetes.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretDestination {
    /// Annotations to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Create the destination Secret. If the Secret already exists this should be set to false.
    pub create: bool,
    /// Labels to apply to the Secret. Requires Create to be set to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Name of the Secret
    pub name: String,
    /// Overwrite the destination Secret if it exists and Create is true. This is useful when migrating to VSO from a previous secret deployment strategy.
    pub overwrite: bool,
    /// Transformation provides configuration for transforming the secret data before it is stored in the Destination.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transformation: Option<VaultDynamicSecretDestinationTransformation>,
    /// Type of Kubernetes Secret. Requires Create to be set to true. Defaults to Opaque.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// Transformation provides configuration for transforming the secret data before it is stored in the Destination.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretDestinationTransformation {
    /// ExcludeRaw data from the destination Secret. Exclusion policy can be set globally by including 'exclude-raw` in the '--global-transformation-options' command line flag. If set, the command line flag always takes precedence over this configuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "excludeRaw")]
    pub exclude_raw: Option<bool>,
    /// Excludes contains regex patterns used to filter top-level source secret data fields for exclusion from the final K8s Secret data. These pattern filters are never applied to templated fields as defined in Templates. They are always applied before any inclusion patterns. To exclude all source secret data fields, you can configure the single pattern ".*".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    /// Includes contains regex patterns used to filter top-level source secret data fields for inclusion in the final K8s Secret data. These pattern filters are never applied to templated fields as defined in Templates. They are always applied last.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
    /// Templates maps a template name to its Template. Templates are always included in the rendered K8s Secret, and take precedence over templates defined in a SecretTransformation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<BTreeMap<String, VaultDynamicSecretDestinationTransformationTemplates>>,
    /// TransformationRefs contain references to template configuration from SecretTransformation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transformationRefs")]
    pub transformation_refs: Option<Vec<VaultDynamicSecretDestinationTransformationTransformationRefs>>,
}

/// Templates maps a template name to its Template. Templates are always included in the rendered K8s Secret, and take precedence over templates defined in a SecretTransformation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretDestinationTransformationTemplates {
    /// Name of the Template
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Text contains the Go text template format. The template references attributes from the data structure of the source secret. Refer to https://pkg.go.dev/text/template for more information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// TransformationRef contains the configuration for accessing templates from an SecretTransformation resource. TransformationRefs can be shared across all syncable secret custom resources.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretDestinationTransformationTransformationRefs {
    /// IgnoreExcludes controls whether to use the SecretTransformation's Excludes data key filters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreExcludes")]
    pub ignore_excludes: Option<bool>,
    /// IgnoreIncludes controls whether to use the SecretTransformation's Includes data key filters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreIncludes")]
    pub ignore_includes: Option<bool>,
    /// Name of the SecretTransformation resource.
    pub name: String,
    /// Namespace of the SecretTransformation resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// TemplateRefs map to a Template found in this TransformationRef. If empty, then all templates from the SecretTransformation will be rendered to the K8s Secret.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "templateRefs")]
    pub template_refs: Option<Vec<VaultDynamicSecretDestinationTransformationTransformationRefsTemplateRefs>>,
}

/// TemplateRef points to templating text that is stored in a SecretTransformation custom resource.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretDestinationTransformationTransformationRefsTemplateRefs {
    /// KeyOverride to the rendered template in the Destination secret. If Key is empty, then the Key from reference spec will be used. Set this to override the Key set from the reference spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keyOverride")]
    pub key_override: Option<String>,
    /// Name of the Template in SecretTransformationSpec.Templates. the rendered secret data.
    pub name: String,
}

/// VaultDynamicSecretSpec defines the desired state of VaultDynamicSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultDynamicSecretRequestHttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretRolloutRestartTargets {
    pub kind: VaultDynamicSecretRolloutRestartTargetsKind,
    pub name: String,
}

/// RolloutRestartTarget provides the configuration required to perform a rollout-restart of the supported resources upon Vault Secret rotation. The rollout-restart is triggered by patching the target resource's 'spec.template.metadata.annotations' to include 'vso.secrets.hashicorp.com/restartedAt' with a timestamp value of when the trigger was executed. E.g. vso.secrets.hashicorp.com/restartedAt: "2023-03-23T13:39:31Z" 
///  Supported resources: Deployment, DaemonSet, StatefulSet
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VaultDynamicSecretRolloutRestartTargetsKind {
    Deployment,
    DaemonSet,
    StatefulSet,
}

/// VaultDynamicSecretStatus defines the observed state of VaultDynamicSecret
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretStatus {
    /// LastGeneration is the Generation of the last reconciled resource.
    #[serde(rename = "lastGeneration")]
    pub last_generation: i64,
    /// LastRenewalTime of the last successful secret lease renewal.
    #[serde(rename = "lastRenewalTime")]
    pub last_renewal_time: i64,
    /// LastRuntimePodUID used for tracking the transition from one Pod to the next. It is used to mitigate the effects of a Vault lease renewal storm.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastRuntimePodUID")]
    pub last_runtime_pod_uid: Option<String>,
    /// SecretLease for the Vault secret.
    #[serde(rename = "secretLease")]
    pub secret_lease: VaultDynamicSecretStatusSecretLease,
    /// SecretMAC used when deciding whether new Vault secret data should be synced. 
    ///  The controller will compare the "new" Vault secret data to this value using HMAC, if they are different, then the data will be synced to the Destination. 
    ///  The SecretMac is also used to detect drift in the Destination Secret's Data. If drift is detected the data will be synced to the Destination. SecretMAC will only be stored when VaultDynamicSecretSpec.AllowStaticCreds is true.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretMAC")]
    pub secret_mac: Option<String>,
    /// StaticCredsMetaData contains the static creds response meta-data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "staticCredsMetaData")]
    pub static_creds_meta_data: Option<VaultDynamicSecretStatusStaticCredsMetaData>,
}

/// SecretLease for the Vault secret.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretStatusSecretLease {
    /// LeaseDuration of the Vault secret.
    pub duration: i64,
    /// ID of the Vault secret.
    pub id: String,
    /// Renewable Vault secret lease
    pub renewable: bool,
    /// RequestID of the Vault secret request.
    #[serde(rename = "requestID")]
    pub request_id: String,
}

/// StaticCredsMetaData contains the static creds response meta-data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VaultDynamicSecretStatusStaticCredsMetaData {
    /// LastVaultRotation represents the last time Vault rotated the password
    #[serde(rename = "lastVaultRotation")]
    pub last_vault_rotation: i64,
    /// RotationPeriod is number in seconds between each rotation, effectively a "time to live". This value is compared to the LastVaultRotation to determine if a password needs to be rotated
    #[serde(rename = "rotationPeriod")]
    pub rotation_period: i64,
    /// RotationSchedule is a "cron style" string representing the allowed schedule for each rotation. e.g. "1 0 * * *" would rotate at one minute past midnight (00:01) every day.
    #[serde(rename = "rotationSchedule")]
    pub rotation_schedule: String,
    /// TTL is the seconds remaining before the next rotation.
    pub ttl: i64,
}

