// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeshop/testkube-operator/tests.testkube.io/v1/testtriggers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "tests.testkube.io", version = "v1", kind = "TestTrigger", plural = "testtriggers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct TestTriggerSpec {
    /// Action represents what needs to be executed for selected Execution
    pub action: TestTriggerAction,
    /// ConcurrencyPolicy defines concurrency policy for selected Execution
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrencyPolicy")]
    pub concurrency_policy: Option<TestTriggerConcurrencyPolicy>,
    /// What resource conditions should be matched
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "conditionSpec")]
    pub condition_spec: Option<TestTriggerConditionSpec>,
    /// Delay is a duration string which specifies how long should the test be delayed after a trigger is matched
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// whether test trigger is disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// On which Event for a Resource should an Action be triggered
    pub event: TestTriggerEvent,
    /// Execution identifies for which test execution should an Action be executed
    pub execution: TestTriggerExecution,
    /// What resource probes should be matched
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "probeSpec")]
    pub probe_spec: Option<TestTriggerProbeSpec>,
    /// For which Resource do we monitor Event which triggers an Action on certain conditions
    pub resource: TestTriggerResource,
    /// ResourceSelector identifies which Kubernetes Objects should be watched
    #[serde(rename = "resourceSelector")]
    pub resource_selector: TestTriggerResourceSelector,
    /// TestSelector identifies on which Testkube Kubernetes Objects an Action should be taken
    #[serde(rename = "testSelector")]
    pub test_selector: TestTriggerTestSelector,
}

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerAction {
    #[serde(rename = "run")]
    Run,
}

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerConcurrencyPolicy {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "forbid")]
    Forbid,
    #[serde(rename = "replace")]
    Replace,
}

/// What resource conditions should be matched
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerConditionSpec {
    /// list of test trigger conditions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TestTriggerConditionSpecConditions>>,
    /// duration in seconds the test trigger waits between condition check
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// duration in seconds the test trigger waits for conditions, until its stopped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

/// TestTriggerCondition is used for definition of the condition for test triggers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestTriggerConditionSpecConditions {
    /// test trigger condition reason
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// TestTriggerConditionStatuses defines condition statuses for test triggers
    pub status: TestTriggerConditionSpecConditionsStatus,
    /// duration in seconds in the past from current time when the condition is still valid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// test trigger condition
    #[serde(rename = "type")]
    pub r#type: String,
}

/// TestTriggerCondition is used for definition of the condition for test triggers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerConditionSpecConditionsStatus {
    True,
    False,
    Unknown,
}

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerEvent {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "deployment-scale-update")]
    DeploymentScaleUpdate,
    #[serde(rename = "deployment-image-update")]
    DeploymentImageUpdate,
    #[serde(rename = "deployment-env-update")]
    DeploymentEnvUpdate,
    #[serde(rename = "deployment-containers-modified")]
    DeploymentContainersModified,
    #[serde(rename = "event-start-test")]
    EventStartTest,
    #[serde(rename = "event-end-test-success")]
    EventEndTestSuccess,
    #[serde(rename = "event-end-test-failed")]
    EventEndTestFailed,
    #[serde(rename = "event-end-test-aborted")]
    EventEndTestAborted,
    #[serde(rename = "event-end-test-timeout")]
    EventEndTestTimeout,
    #[serde(rename = "event-start-testsuite")]
    EventStartTestsuite,
    #[serde(rename = "event-end-testsuite-success")]
    EventEndTestsuiteSuccess,
    #[serde(rename = "event-end-testsuite-failed")]
    EventEndTestsuiteFailed,
    #[serde(rename = "event-end-testsuite-aborted")]
    EventEndTestsuiteAborted,
    #[serde(rename = "event-end-testsuite-timeout")]
    EventEndTestsuiteTimeout,
    #[serde(rename = "event-queue-testworkflow")]
    EventQueueTestworkflow,
    #[serde(rename = "event-start-testworkflow")]
    EventStartTestworkflow,
    #[serde(rename = "event-end-testworkflow-success")]
    EventEndTestworkflowSuccess,
    #[serde(rename = "event-end-testworkflow-failed")]
    EventEndTestworkflowFailed,
    #[serde(rename = "event-end-testworkflow-aborted")]
    EventEndTestworkflowAborted,
    #[serde(rename = "event-created")]
    EventCreated,
    #[serde(rename = "event-updated")]
    EventUpdated,
    #[serde(rename = "event-deleted")]
    EventDeleted,
}

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerExecution {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "testsuite")]
    Testsuite,
    #[serde(rename = "testworkflow")]
    Testworkflow,
}

/// What resource probes should be matched
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerProbeSpec {
    /// duration in seconds the test trigger waits between probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// list of test trigger probes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probes: Option<Vec<TestTriggerProbeSpecProbes>>,
    /// duration in seconds the test trigger waits for probes, until its stopped
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

/// TestTriggerProbe is used for definition of the probe for test triggers
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerProbeSpecProbes {
    /// test trigger condition probe headers to submit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    /// test trigger condition probe host, default is pod ip or service name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// test trigger condition probe path to check, default is /
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// test trigger condition probe port to connect
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// test trigger condition probe scheme to connect to host, default is http
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

/// TestTriggerSpec defines the desired state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TestTriggerResource {
    #[serde(rename = "pod")]
    Pod,
    #[serde(rename = "deployment")]
    Deployment,
    #[serde(rename = "statefulset")]
    Statefulset,
    #[serde(rename = "daemonset")]
    Daemonset,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "configmap")]
    Configmap,
}

/// ResourceSelector identifies which Kubernetes Objects should be watched
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerResourceSelector {
    /// LabelSelector is used to identify a group of Kubernetes Objects based on their metadata labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<TestTriggerResourceSelectorLabelSelector>,
    /// Name selector is used to identify a Kubernetes Object based on the metadata name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// kubernetes resource name regex
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameRegex")]
    pub name_regex: Option<String>,
    /// Namespace of the Kubernetes object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// LabelSelector is used to identify a group of Kubernetes Objects based on their metadata labels
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerResourceSelectorLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TestTriggerResourceSelectorLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerResourceSelectorLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TestSelector identifies on which Testkube Kubernetes Objects an Action should be taken
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerTestSelector {
    /// LabelSelector is used to identify a group of Kubernetes Objects based on their metadata labels
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<TestTriggerTestSelectorLabelSelector>,
    /// Name selector is used to identify a Kubernetes Object based on the metadata name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// kubernetes resource name regex
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameRegex")]
    pub name_regex: Option<String>,
    /// Namespace of the Kubernetes object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// LabelSelector is used to identify a group of Kubernetes Objects based on their metadata labels
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerTestSelectorLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<TestTriggerTestSelectorLabelSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels
    /// map is equivalent to an element of matchExpressions, whose key field is "key", the
    /// operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerTestSelectorLabelSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// TestTriggerStatus defines the observed state of TestTrigger
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestTriggerStatus {
}

