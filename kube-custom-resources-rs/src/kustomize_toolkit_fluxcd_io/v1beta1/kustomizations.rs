// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/fluxcd/kustomize-controller/kustomize.toolkit.fluxcd.io/v1beta1/kustomizations.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// KustomizationSpec defines the desired state of a kustomization.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "kustomize.toolkit.fluxcd.io", version = "v1beta1", kind = "Kustomization", plural = "kustomizations")]
#[kube(namespaced)]
#[kube(status = "KustomizationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct KustomizationSpec {
    /// Decrypt Kubernetes secrets before applying them on the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decryption: Option<KustomizationDecryption>,
    /// DependsOn may contain a meta.NamespacedObjectReference slice
    /// with references to Kustomization resources that must be ready before this
    /// Kustomization can be reconciled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dependsOn")]
    pub depends_on: Option<Vec<KustomizationDependsOn>>,
    /// Force instructs the controller to recreate resources
    /// when patching fails due to an immutable field change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// A list of resources to be included in the health assessment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthChecks")]
    pub health_checks: Option<Vec<KustomizationHealthChecks>>,
    /// Images is a list of (image name, new name, new tag or digest)
    /// for changing image names, tags or digests. This can also be achieved with a
    /// patch, but this operator is simpler to specify.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<KustomizationImages>>,
    /// The interval at which to reconcile the Kustomization.
    pub interval: String,
    /// The KubeConfig for reconciling the Kustomization on a remote cluster.
    /// When specified, KubeConfig takes precedence over ServiceAccountName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeConfig")]
    pub kube_config: Option<KustomizationKubeConfig>,
    /// Strategic merge and JSON patches, defined as inline YAML objects,
    /// capable of targeting objects based on kind, label and annotation selectors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<KustomizationPatches>>,
    /// JSON 6902 patches, defined as inline YAML objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "patchesJson6902")]
    pub patches_json6902: Option<Vec<KustomizationPatchesJson6902>>,
    /// Strategic merge patches, defined as inline YAML objects.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "patchesStrategicMerge")]
    pub patches_strategic_merge: Option<Vec<BTreeMap<String, serde_json::Value>>>,
    /// Path to the directory containing the kustomization.yaml file, or the
    /// set of plain YAMLs a kustomization.yaml should be generated for.
    /// Defaults to 'None', which translates to the root path of the SourceRef.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// PostBuild describes which actions to perform on the YAML manifest
    /// generated by building the kustomize overlay.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postBuild")]
    pub post_build: Option<KustomizationPostBuild>,
    /// Prune enables garbage collection.
    pub prune: bool,
    /// The interval at which to retry a previously failed reconciliation.
    /// When not specified, the controller uses the KustomizationSpec.Interval
    /// value to retry failures.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
    /// The name of the Kubernetes service account to impersonate
    /// when reconciling this Kustomization.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
    /// Reference of the source where the kustomization file is.
    #[serde(rename = "sourceRef")]
    pub source_ref: KustomizationSourceRef,
    /// This flag tells the controller to suspend subsequent kustomize executions,
    /// it does not apply to already started executions. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// TargetNamespace sets or overrides the namespace in the
    /// kustomization.yaml file.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetNamespace")]
    pub target_namespace: Option<String>,
    /// Timeout for validation, apply and health checking operations.
    /// Defaults to 'Interval' duration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Validate the Kubernetes objects before applying them on the cluster.
    /// The validation strategy can be 'client' (local dry-run), 'server'
    /// (APIServer dry-run) or 'none'.
    /// When 'Force' is 'true', validation will fallback to 'client' if set to
    /// 'server' because server-side validation is not supported in this scenario.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<KustomizationValidation>,
}

/// Decrypt Kubernetes secrets before applying them on the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KustomizationDecryption {
    /// Provider is the name of the decryption engine.
    pub provider: KustomizationDecryptionProvider,
    /// The secret name containing the private OpenPGP keys used for decryption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<KustomizationDecryptionSecretRef>,
}

/// Decrypt Kubernetes secrets before applying them on the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KustomizationDecryptionProvider {
    #[serde(rename = "sops")]
    Sops,
}

/// The secret name containing the private OpenPGP keys used for decryption.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationDecryptionSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// NamespacedObjectReference contains enough information to locate the referenced Kubernetes resource object in any
/// namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationDependsOn {
    /// Name of the referent.
    pub name: String,
    /// Namespace of the referent, when not specified it acts as LocalObjectReference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// NamespacedObjectKindReference contains enough information to locate the typed referenced Kubernetes resource object
/// in any namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationHealthChecks {
    /// API version of the referent, if not specified the Kubernetes preferred version will be used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent.
    pub kind: String,
    /// Name of the referent.
    pub name: String,
    /// Namespace of the referent, when not specified it acts as LocalObjectReference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Image contains an image name, a new name, a new tag or digest, which will replace the original name and tag.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationImages {
    /// Digest is the value used to replace the original image tag.
    /// If digest is present NewTag value is ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Name is a tag-less image name.
    pub name: String,
    /// NewName is the value used to replace the original name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newName")]
    pub new_name: Option<String>,
    /// NewTag is the value used to replace the original tag.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newTag")]
    pub new_tag: Option<String>,
}

/// The KubeConfig for reconciling the Kustomization on a remote cluster.
/// When specified, KubeConfig takes precedence over ServiceAccountName.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationKubeConfig {
    /// SecretRef holds the name to a secret that contains a 'value' key with
    /// the kubeconfig file as the value. It must be in the same namespace as
    /// the Kustomization.
    /// It is recommended that the kubeconfig is self-contained, and the secret
    /// is regularly updated if credentials such as a cloud-access-token expire.
    /// Cloud specific `cmd-path` auth helpers will not function without adding
    /// binaries and credentials to the Pod that is responsible for reconciling
    /// the Kustomization.
    #[serde(rename = "secretRef")]
    pub secret_ref: KustomizationKubeConfigSecretRef,
}

/// SecretRef holds the name to a secret that contains a 'value' key with
/// the kubeconfig file as the value. It must be in the same namespace as
/// the Kustomization.
/// It is recommended that the kubeconfig is self-contained, and the secret
/// is regularly updated if credentials such as a cloud-access-token expire.
/// Cloud specific `cmd-path` auth helpers will not function without adding
/// binaries and credentials to the Pod that is responsible for reconciling
/// the Kustomization.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationKubeConfigSecretRef {
    /// Name of the referent.
    pub name: String,
}

/// Patch contains an inline StrategicMerge or JSON6902 patch, and the target the patch should
/// be applied to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationPatches {
    /// Patch contains an inline StrategicMerge patch or an inline JSON6902 patch with
    /// an array of operation objects.
    pub patch: String,
    /// Target points to the resources that the patch document should be applied to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<KustomizationPatchesTarget>,
}

/// Target points to the resources that the patch document should be applied to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationPatchesTarget {
    /// AnnotationSelector is a string that follows the label selection expression
    /// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#api
    /// It matches with the resource annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelector")]
    pub annotation_selector: Option<String>,
    /// Group is the API group to select resources from.
    /// Together with Version and Kind it is capable of unambiguously identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind of the API Group to select resources from.
    /// Together with Group and Version it is capable of unambiguously
    /// identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// LabelSelector is a string that follows the label selection expression
    /// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#api
    /// It matches with the resource labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<String>,
    /// Name to match resources with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace to select resources from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Version of the API Group to select resources from.
    /// Together with Group and Kind it is capable of unambiguously identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// JSON6902Patch contains a JSON6902 patch and the target the patch should be applied to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationPatchesJson6902 {
    /// Patch contains the JSON6902 patch document with an array of operation objects.
    pub patch: Vec<KustomizationPatchesJson6902Patch>,
    /// Target points to the resources that the patch document should be applied to.
    pub target: KustomizationPatchesJson6902Target,
}

/// JSON6902 is a JSON6902 operation object.
/// https://datatracker.ietf.org/doc/html/rfc6902#section-4
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KustomizationPatchesJson6902Patch {
    /// From contains a JSON-pointer value that references a location within the target document where the operation is
    /// performed. The meaning of the value depends on the value of Op, and is NOT taken into account by all operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Op indicates the operation to perform. Its value MUST be one of "add", "remove", "replace", "move", "copy", or
    /// "test".
    /// https://datatracker.ietf.org/doc/html/rfc6902#section-4
    pub op: KustomizationPatchesJson6902PatchOp,
    /// Path contains the JSON-pointer value that references a location within the target document where the operation
    /// is performed. The meaning of the value depends on the value of Op.
    pub path: String,
    /// Value contains a valid JSON structure. The meaning of the value depends on the value of Op, and is NOT taken into
    /// account by all operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// JSON6902 is a JSON6902 operation object.
/// https://datatracker.ietf.org/doc/html/rfc6902#section-4
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KustomizationPatchesJson6902PatchOp {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "copy")]
    Copy,
}

/// Target points to the resources that the patch document should be applied to.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationPatchesJson6902Target {
    /// AnnotationSelector is a string that follows the label selection expression
    /// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#api
    /// It matches with the resource annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelector")]
    pub annotation_selector: Option<String>,
    /// Group is the API group to select resources from.
    /// Together with Version and Kind it is capable of unambiguously identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Kind of the API Group to select resources from.
    /// Together with Group and Version it is capable of unambiguously
    /// identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// LabelSelector is a string that follows the label selection expression
    /// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#api
    /// It matches with the resource labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<String>,
    /// Name to match resources with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace to select resources from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Version of the API Group to select resources from.
    /// Together with Group and Kind it is capable of unambiguously identifying and/or selecting resources.
    /// https://github.com/kubernetes/community/blob/master/contributors/design-proposals/api-machinery/api-group.md
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// PostBuild describes which actions to perform on the YAML manifest
/// generated by building the kustomize overlay.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationPostBuild {
    /// Substitute holds a map of key/value pairs.
    /// The variables defined in your YAML manifests
    /// that match any of the keys defined in the map
    /// will be substituted with the set value.
    /// Includes support for bash string replacement functions
    /// e.g. ${var:=default}, ${var:position} and ${var/substring/replacement}.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitute: Option<BTreeMap<String, String>>,
    /// SubstituteFrom holds references to ConfigMaps and Secrets containing
    /// the variables and their values to be substituted in the YAML manifests.
    /// The ConfigMap and the Secret data keys represent the var names and they
    /// must match the vars declared in the manifests for the substitution to happen.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "substituteFrom")]
    pub substitute_from: Option<Vec<KustomizationPostBuildSubstituteFrom>>,
}

/// SubstituteReference contains a reference to a resource containing
/// the variables name and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KustomizationPostBuildSubstituteFrom {
    /// Kind of the values referent, valid values are ('Secret', 'ConfigMap').
    pub kind: KustomizationPostBuildSubstituteFromKind,
    /// Name of the values referent. Should reside in the same namespace as the
    /// referring resource.
    pub name: String,
}

/// SubstituteReference contains a reference to a resource containing
/// the variables name and value.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KustomizationPostBuildSubstituteFromKind {
    Secret,
    ConfigMap,
}

/// Reference of the source where the kustomization file is.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KustomizationSourceRef {
    /// API version of the referent
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent
    pub kind: KustomizationSourceRefKind,
    /// Name of the referent
    pub name: String,
    /// Namespace of the referent, defaults to the Kustomization namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Reference of the source where the kustomization file is.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KustomizationSourceRefKind {
    GitRepository,
    Bucket,
}

/// KustomizationSpec defines the desired state of a kustomization.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KustomizationValidation {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
}

/// KustomizationStatus defines the observed state of a kustomization.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The last successfully applied revision.
    /// The revision format for Git sources is <branch|tag>/<commit-sha>.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAppliedRevision")]
    pub last_applied_revision: Option<String>,
    /// LastAttemptedRevision is the revision of the last reconciliation attempt.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastAttemptedRevision")]
    pub last_attempted_revision: Option<String>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedGeneration is the last reconciled generation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// The last successfully applied revision metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<KustomizationStatusSnapshot>,
}

/// The last successfully applied revision metadata.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationStatusSnapshot {
    /// The manifests sha1 checksum.
    pub checksum: String,
    /// A list of Kubernetes kinds grouped by namespace.
    pub entries: Vec<KustomizationStatusSnapshotEntries>,
}

/// Snapshot holds the metadata of namespaced
/// Kubernetes objects
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KustomizationStatusSnapshotEntries {
    /// The list of Kubernetes kinds.
    pub kinds: BTreeMap<String, String>,
    /// The namespace of this entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

