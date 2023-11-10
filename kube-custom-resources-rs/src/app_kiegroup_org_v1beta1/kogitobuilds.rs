// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kiegroup/kogito-operator/app.kiegroup.org/v1beta1/kogitobuilds.yaml --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// KogitoBuildSpec defines the desired state of KogitoBuild.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "app.kiegroup.org", version = "v1beta1", kind = "KogitoBuild", plural = "kogitobuilds")]
#[kube(namespaced)]
#[kube(status = "KogitoBuildStatus")]
#[kube(schema = "disabled")]
pub struct KogitoBuildSpec {
    /// Artifact contains override information for building the Maven artifact (used for Local Source builds). 
    ///  You might want to override this information when building from decisions, rules or process files. In this scenario the Kogito Images will generate a new Java project for you underneath. This information will be used to generate this project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<KogitoBuildArtifact>,
    /// Image used to build the Kogito Service from source (Local and Remote). 
    ///  If not defined the operator will use image provided by the Kogito Team based on the "Runtime" field. 
    ///  Example: "quay.io/kiegroup/kogito-jvm-builder:latest". 
    ///  On OpenShift an ImageStream will be created in the current namespace pointing to the given image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "buildImage")]
    pub build_image: Option<String>,
    /// DisableIncremental indicates that source to image builds should NOT be incremental. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableIncremental")]
    pub disable_incremental: Option<bool>,
    /// If set to true will print the logs for downloading/uploading of maven dependencies. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableMavenDownloadOutput")]
    pub enable_maven_download_output: Option<bool>,
    /// Environment variables used during build time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<KogitoBuildEnv>>,
    /// Information about the git repository where the Kogito Service source code resides. 
    ///  Ignored for binary builds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitSource")]
    pub git_source: Option<KogitoBuildGitSource>,
    /// Maven Mirror URL to be used during source-to-image builds (Local and Remote) to considerably increase build speed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mavenMirrorURL")]
    pub maven_mirror_url: Option<String>,
    /// Native indicates if the Kogito Service built should be compiled to run on native mode when Runtime is Quarkus (Source to Image build only). 
    ///  For more information, see https://www.graalvm.org/docs/reference-manual/aot-compilation/.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    /// Resources Requirements for builder pods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<KogitoBuildResources>,
    /// Which runtime Kogito service base image to use when building the Kogito service. If "BuildImage" is set, this value is ignored by the operator. Default value: quarkus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime: Option<KogitoBuildRuntime>,
    /// Image used as the base image for the final Kogito service. This image only has the required packages to run the application. 
    ///  For example: quarkus based services will have only JVM installed, native services only the packages required by the OS. 
    ///  If not defined the operator will use image provided by the Kogito Team based on the "Runtime" field. 
    ///  Example: "quay.io/kiegroup/kogito-jvm-builder:latest". 
    ///  On OpenShift an ImageStream will be created in the current namespace pointing to the given image.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "runtimeImage")]
    pub runtime_image: Option<String>,
    /// Set this field targeting the desired KogitoRuntime when this KogitoBuild instance has a different name than the KogitoRuntime. 
    ///  By default this KogitoBuild instance will generate a final image named after its own name (.metadata.name). 
    ///  On OpenShift, an ImageStream will be created causing a redeployment on any KogitoRuntime with the same name. On Kubernetes, the final image will be pushed to the KogitoRuntime deployment. 
    ///  If you have multiple KogitoBuild instances (let's say BinaryBuildType and Remote Source), you might need that both target the same KogitoRuntime. Both KogitoBuilds will update the same ImageStream or generate a final image to the same KogitoRuntime deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetKogitoRuntime")]
    pub target_kogito_runtime: Option<String>,
    /// Sets the type of build that this instance will handle: 
    ///  Binary - takes an uploaded binary file already compiled and creates a Kogito service image from it. 
    ///  RemoteSource - pulls the source code from a Git repository, builds the binary and then the final Kogito service image. 
    ///  LocalSource - takes an uploaded resource file such as DRL (rules), DMN (decision) or BPMN (process), builds the binary and the final Kogito service image.
    #[serde(rename = "type")]
    pub r#type: KogitoBuildType,
    /// WebHooks secrets for source to image builds based on Git repositories (Remote Sources).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "webHooks")]
    pub web_hooks: Option<Vec<KogitoBuildWebHooks>>,
}

/// Artifact contains override information for building the Maven artifact (used for Local Source builds). 
///  You might want to override this information when building from decisions, rules or process files. In this scenario the Kogito Images will generate a new Java project for you underneath. This information will be used to generate this project.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildArtifact {
    /// Indicates the unique base name of the primary artifact being generated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "artifactId")]
    pub artifact_id: Option<String>,
    /// Indicates the unique identifier of the organization or group that created the project.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupId")]
    pub group_id: Option<String>,
    /// Indicates the version of the artifact generated by the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<KogitoBuildEnvValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnvValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<KogitoBuildEnvValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<KogitoBuildEnvValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<KogitoBuildEnvValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<KogitoBuildEnvValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnvValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnvValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnvValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildEnvValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Information about the git repository where the Kogito Service source code resides. 
///  Ignored for binary builds.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildGitSource {
    /// Context/subdirectory where the code is located, relative to the repo root.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contextDir")]
    pub context_dir: Option<String>,
    /// Branch to use in the Git repository.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Git URI for the s2i source.
    pub uri: String,
}

/// Resources Requirements for builder pods.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// KogitoBuildSpec defines the desired state of KogitoBuild.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KogitoBuildRuntime {
    #[serde(rename = "quarkus")]
    Quarkus,
    #[serde(rename = "springboot")]
    Springboot,
}

/// KogitoBuildSpec defines the desired state of KogitoBuild.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KogitoBuildType {
    Binary,
    RemoteSource,
    LocalSource,
}

/// WebHookSecret Secret to use for a given webHook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildWebHooks {
    /// Secret value for webHook
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// WebHook type, either GitHub or Generic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<KogitoBuildWebHooksType>,
}

/// WebHookSecret Secret to use for a given webHook.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KogitoBuildWebHooksType {
    GitHub,
    Generic,
}

/// KogitoBuildStatus defines the observed state of KogitoBuild.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildStatus {
    /// History of builds
    pub builds: KogitoBuildStatusBuilds,
    /// History of conditions for the resource
    pub conditions: Vec<KogitoBuildStatusConditions>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestBuild")]
    pub latest_build: Option<String>,
}

/// History of builds
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildStatusBuilds {
    /// Builds have been stopped from executing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<Vec<String>>,
    /// Builds have executed and succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<Vec<String>>,
    /// Builds have been prevented from executing by an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<String>>,
    /// Builds have executed and failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<String>>,
    /// Builds are being created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new: Option<Vec<String>>,
    /// Builds are about to start running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<Vec<String>>,
    /// Builds are running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<Vec<String>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KogitoBuildStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: KogitoBuildStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, type FooStatus struct{ // Represents the observations of a foo's current state. // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" // +patchMergeKey=type // +patchStrategy=merge // +listType=map // +listMapKey=type Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  // other fields }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum KogitoBuildStatusConditionsStatus {
    True,
    False,
    Unknown,
}

