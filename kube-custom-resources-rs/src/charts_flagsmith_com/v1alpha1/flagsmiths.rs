// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Flagsmith/flagsmith-operator/charts.flagsmith.com/v1alpha1/flagsmiths.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// Spec defines the desired state of Flagsmith
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "charts.flagsmith.com", version = "v1alpha1", kind = "Flagsmith", plural = "flagsmiths")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct FlagsmithSpec {
    /// Configuration how to setup the flagsmith api service.
    pub api: BTreeMap<String, serde_json::Value>,
    /// Configuration how to setup the flagsmith frontend service.
    pub frontend: BTreeMap<String, serde_json::Value>,
    /// Configuration how to setup the flagsmith hooks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<BTreeMap<String, serde_json::Value>>,
    /// Configuration how to setup the flagsmith InfluxDB service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influxdb: Option<FlagsmithInfluxdb>,
    /// Configuration how to setup ingress to the flagsmith if flagsmith is using Kubernetes and not OpenShift.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<BTreeMap<String, serde_json::Value>>,
    /// Configuration how to setup the flagsmith metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BTreeMap<String, serde_json::Value>>,
    /// If flagsmith install on OpenShift set value to true otherwise false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openshift: Option<bool>,
    /// Configuration how to setup the flagsmith postgresql service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postgresql: Option<FlagsmithPostgresql>,
    /// Configuration how to setup the flagsmith kubernetes service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<BTreeMap<String, serde_json::Value>>,
}

/// Configuration how to setup the flagsmith InfluxDB service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlagsmithInfluxdb {
    /// Set to true if InfluxDB will be installed. If the value is false InfluxDB will not be installed.
    pub enabled: bool,
}

/// Configuration how to setup the flagsmith postgresql service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FlagsmithPostgresql {
    /// Set to true if PostgreSQL will be installed. If the value is false PostgreSQL will not be installed.
    pub enabled: bool,
}

