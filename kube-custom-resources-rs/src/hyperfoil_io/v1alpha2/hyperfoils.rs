// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Hyperfoil/hyperfoil-operator/hyperfoil.io/v1alpha2/hyperfoils.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// HyperfoilSpec defines the desired state of Hyperfoil.It Configures Hyperfoil Controller and related resources.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "hyperfoil.io", version = "v1alpha2", kind = "Hyperfoil", plural = "hyperfoils")]
#[kube(namespaced)]
#[kube(status = "HyperfoilStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HyperfoilSpec {
    /// AdditionalArgs specifies additional arguments to pass to the Hyperfoil controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalArgs")]
    pub additional_args: Option<Vec<String>>,
    /// Deploy timeout for agents, in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "agentDeployTimeout")]
    pub agent_deploy_timeout: Option<i64>,
    /// Authentication/authorization settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<HyperfoilAuth>,
    /// Controller image. If 'version' is defined, too, the tag is replaced (or appended). Defaults to 'quay.io/hyperfoil/hyperfoil'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Name of the config map and optionally its entry (separated by '/': e.g myconfigmap/log4j2-superverbose.xml) storing Log4j2 configuration file. By default the Controller uses its embedded configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// Name of the PVC hyperfoil should mount for its workdir.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<String>,
    /// Names of config maps and optionally keys (separated by '/') holding hooks that run after the run finishes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "postHooks")]
    pub post_hooks: Option<Vec<String>>,
    /// Names of config maps and optionally keys (separated by '/') holding hooks that run before the run starts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preHooks")]
    pub pre_hooks: Option<Vec<String>>,
    /// Specification of the exposed route. This setting is ignored when Openshift Routes are not available (on vanilla Kubernetes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<HyperfoilRoute>,
    /// List of secrets in this namespace; each entry from those secrets will be mapped
    /// as environment variable, using the key as variable name.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretEnvVars")]
    pub secret_env_vars: Option<Vec<String>>,
    /// Type of the service being exposed. By default this is ClusterIP if Openshift Route resource is available (the route will target this service).
    /// If Openshift Routes are not available (on vanilla Kubernetes) the default is NodePort.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<String>,
    /// If this is set the controller does not start benchmark run right away after hitting
    /// /benchmark/my-benchmark/start ; instead it responds with status 301 and header Location
    /// set to concatenation of this string and 'BENCHMARK=my-benchmark&RUN_ID=xxxx'.
    /// CLI interprets that response as a request to hit CI instance on this URL, assuming that
    /// CI will trigger a new job that will eventually call /benchmark/my-benchmark/start?runId=xxxx
    /// with header 'x-trigger-job'. This is useful if the the CI has to synchronize Hyperfoil
    /// to other benchmarks that don't use this controller instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerUrl")]
    pub trigger_url: Option<String>,
    /// Tag for controller image. Defaults to version matching the operator version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Authentication/authorization settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HyperfoilAuth {
    /// Optional; Name of secret used for basic authentication. Must contain key 'password'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

/// Specification of the exposed route. This setting is ignored when Openshift Routes are not available (on vanilla Kubernetes).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HyperfoilRoute {
    /// Host for the route leading to Controller REST endpoint. Example: hyperfoil.apps.cloud.example.com
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Optional for edge and reencrypt routes, required for passthrough; Name of the secret hosting `tls.crt`, `tls.key` and optionally `ca.crt`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    /// Either 'http' (for plain-text routes - not recommended), 'edge', 'reencrypt' or 'passthrough'
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// HyperfoilStatus defines the observed state of Hyperfoil
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HyperfoilStatus {
    /// RFC 3339 date and time of the last update.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastUpdate")]
    pub last_update: Option<String>,
    /// Human readable explanation for the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// "One of: 'Ready', 'Pending' or 'Error'"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

