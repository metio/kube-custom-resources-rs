// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/tinkerbell/tink/tinkerbell.org/v1alpha2/templates.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.5

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tinkerbell.org", version = "v1alpha2", kind = "Template", plural = "templates")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
pub struct TemplateSpec {
    /// Actions defines the set of actions to be run on a target machine. Actions are run sequentially
    /// in the order they are specified. At least 1 action must be specified. Names of actions
    /// must be unique within a Template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<TemplateActions>>,
    /// Env defines environment variables to be available in all actions. If an action specifies
    /// the same environment variable it will take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
    /// Volumes to be mounted on all actions. If an action specifies the same volume it will take
    /// precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}

/// Action defines an individual action to be run on a target machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemplateActions {
    /// Args are a set of arguments to be passed to the command executed by the container on
    /// launch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Cmd defines the command to use when launching the image. It overrides the default command
    /// of the action. It must be a unix path to an executable program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmd: Option<String>,
    /// Env defines environment variables used when launching the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
    /// Image is an OCI image.
    pub image: String,
    /// Name is a name for the action.
    pub name: String,
    /// Namespace defines the Linux namespaces this container should execute in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<TemplateActionsNamespaces>,
    /// Volumes defines the volumes to mount into the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}

/// Namespace defines the Linux namespaces this container should execute in.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemplateActionsNamespaces {
    /// Network defines the network namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// PID defines the PID namespace
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

