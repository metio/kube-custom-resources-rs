// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeshop/testkube-operator/tests.testkube.io/v3/tests.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// TestSpec defines the desired state of Test
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tests.testkube.io", version = "v3", kind = "Test", plural = "tests")]
#[kube(namespaced)]
#[kube(status = "TestStatus")]
#[kube(schema = "disabled")]
pub struct TestSpec {
    /// test content object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<TestContent>,
    /// test description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// test execution request body
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionRequest")]
    pub execution_request: Option<TestExecutionRequest>,
    /// test name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// schedule in cron job format for scheduled test execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// reference to test source resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// test type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// files to be used from minio uploads
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploads: Option<Vec<String>>,
}

/// test content object
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContent {
    /// test content body
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// repository of test content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<TestContentRepository>,
    /// test type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<TestContentType>,
    /// uri of test content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// repository of test content
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContentRepository {
    /// auth type for git requests
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authType")]
    pub auth_type: Option<TestContentRepositoryAuthType>,
    /// branch/tag name for checkout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// git auth certificate secret for private repositories
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateSecret")]
    pub certificate_secret: Option<String>,
    /// commit id (sha) for checkout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// if needed we can checkout particular path (dir or file) in case of BIG/mono repositories
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// SecretRef is the Testkube internal reference for secret storage in Kubernetes secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenSecret")]
    pub token_secret: Option<TestContentRepositoryTokenSecret>,
    /// VCS repository type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// uri of content file or git directory
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// SecretRef is the Testkube internal reference for secret storage in Kubernetes secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernameSecret")]
    pub username_secret: Option<TestContentRepositoryUsernameSecret>,
    /// if provided we checkout the whole repository and run test from this directory
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workingDir")]
    pub working_dir: Option<String>,
}

/// repository of test content
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestContentRepositoryAuthType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "header")]
    Header,
}

/// SecretRef is the Testkube internal reference for secret storage in Kubernetes secrets
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContentRepositoryTokenSecret {
    /// object key
    pub key: String,
    /// object name
    pub name: String,
}

/// SecretRef is the Testkube internal reference for secret storage in Kubernetes secrets
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestContentRepositoryUsernameSecret {
    /// object key
    pub key: String,
    /// object name
    pub name: String,
}

/// test content object
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestContentType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "file-uri")]
    FileUri,
    #[serde(rename = "git-file")]
    GitFile,
    #[serde(rename = "git-dir")]
    GitDir,
    #[serde(rename = "git")]
    Git,
}

/// test execution request body
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequest {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDeadlineSeconds")]
    pub active_deadline_seconds: Option<i64>,
    /// additional executor binary arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// usage mode for arguments
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "argsMode")]
    pub args_mode: Option<TestExecutionRequestArgsMode>,
    /// artifact request body with test artifacts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "artifactRequest")]
    pub artifact_request: Option<TestExecutionRequestArtifactRequest>,
    /// executor binary command
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// cron job template extensions
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cronJobTemplate")]
    pub cron_job_template: Option<String>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cronJobTemplateReference")]
    pub cron_job_template_reference: Option<String>,
    /// config map references
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envConfigMaps")]
    pub env_config_maps: Option<Vec<TestExecutionRequestEnvConfigMaps>>,
    /// secret references
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envSecrets")]
    pub env_secrets: Option<Vec<TestExecutionRequestEnvSecrets>>,
    /// Environment variables passed to executor. Deprecated: use Basic Variables instead
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub envs: Option<BTreeMap<String, String>>,
    /// execute post run script before scraping (prebuilt executor only)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executePostRunScriptBeforeScraping")]
    pub execute_post_run_script_before_scraping: Option<bool>,
    /// test execution labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "executionLabels")]
    pub execution_labels: Option<BTreeMap<String, String>>,
    /// http proxy for executor containers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpProxy")]
    pub http_proxy: Option<String>,
    /// https proxy for executor containers
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsProxy")]
    pub https_proxy: Option<String>,
    /// container executor image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// container executor image pull secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<TestExecutionRequestImagePullSecrets>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isVariablesFileUploaded")]
    pub is_variables_file_uploaded: Option<bool>,
    /// job template extensions
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobTemplate")]
    pub job_template: Option<String>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobTemplateReference")]
    pub job_template_reference: Option<String>,
    /// test execution custom name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// test kubernetes namespace (\"testkube\" when not set)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// negative test will fail the execution if it is a success and it will succeed if it is a failure
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "negativeTest")]
    pub negative_test: Option<bool>,
    /// test execution number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// script to run after test execution
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postRunScript")]
    pub post_run_script: Option<String>,
    /// script to run before test execution
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preRunScript")]
    pub pre_run_script: Option<String>,
    /// pvc template extensions
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvcTemplate")]
    pub pvc_template: Option<String>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pvcTemplateReference")]
    pub pvc_template_reference: Option<String>,
    /// scraper template extensions
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scraperTemplate")]
    pub scraper_template: Option<String>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scraperTemplateReference")]
    pub scraper_template_reference: Option<String>,
    /// Execution variables passed to executor from secrets. Deprecated: use Secret Variables instead
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretEnvs")]
    pub secret_envs: Option<BTreeMap<String, String>>,
    /// whether to start execution sync or async
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync: Option<bool>,
    /// test secret uuid
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "testSecretUUID")]
    pub test_secret_uuid: Option<String>,
    /// unique test suite name (CRD Test suite name), if it's run as a part of test suite
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "testSuiteName")]
    pub test_suite_name: Option<String>,
    /// test suite secret uuid, if it's run as a part of test suite
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "testSuiteSecretUUID")]
    pub test_suite_secret_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<BTreeMap<String, TestExecutionRequestVariables>>,
    /// variables file content - need to be in format for particular executor (e.g. postman envs file)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "variablesFile")]
    pub variables_file: Option<String>,
}

/// test execution request body
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestExecutionRequestArgsMode {
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "override")]
    Override,
}

/// artifact request body with test artifacts
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestArtifactRequest {
    /// artifact directories for scraping
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dirs: Option<Vec<String>>,
    /// don't use a separate folder for execution artifacts
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "omitFolderPerExecution")]
    pub omit_folder_per_execution: Option<bool>,
    /// artifact bucket storage
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<String>,
    /// artifact storage class name for container executor
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storageClassName")]
    pub storage_class_name: Option<String>,
    /// artifact volume mount path for container executor
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeMountPath")]
    pub volume_mount_path: Option<String>,
}

/// Reference to env resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestEnvConfigMaps {
    /// whether we shoud map to variables from resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapToVariables")]
    pub map_to_variables: Option<bool>,
    /// whether we shoud mount resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<bool>,
    /// where we shoud mount resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
    pub reference: TestExecutionRequestEnvConfigMapsReference,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestEnvConfigMapsReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Reference to env resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestEnvSecrets {
    /// whether we shoud map to variables from resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mapToVariables")]
    pub map_to_variables: Option<bool>,
    /// whether we shoud mount resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<bool>,
    /// where we shoud mount resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountPath")]
    pub mount_path: Option<String>,
    /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
    pub reference: TestExecutionRequestEnvSecretsReference,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestEnvSecretsReference {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestImagePullSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariables {
    /// variable name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// variable type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    /// variable string value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// or load it from var source
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<TestExecutionRequestVariablesValueFrom>,
}

/// or load it from var source
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariablesValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<TestExecutionRequestVariablesValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<TestExecutionRequestVariablesValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<TestExecutionRequestVariablesValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<TestExecutionRequestVariablesValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariablesValueFromConfigMapKeyRef {
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariablesValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariablesValueFromResourceFieldRef {
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
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestExecutionRequestVariablesValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// TestStatus defines the observed state of Test
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestStatus {
    /// latest execution result
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "latestExecution")]
    pub latest_execution: Option<TestStatusLatestExecution>,
}

/// latest execution result
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestStatusLatestExecution {
    /// test end time
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<String>,
    /// execution id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// execution number
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// test start time
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TestStatusLatestExecutionStatus>,
}

/// latest execution result
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestStatusLatestExecutionStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "timeout")]
    Timeout,
}

