// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/stackabletech/superset-operator/superset.stackable.tech/v1alpha1/druidconnections.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// The DruidConnection resource can be used to automatically deploy a Druid datasource in Superset. Learn more about it in the [Superset operator usage guide](https://docs.stackable.tech/home/nightly/superset/usage-guide/connecting-druid).
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "superset.stackable.tech", version = "v1alpha1", kind = "DruidConnection", plural = "druidconnections")]
#[kube(namespaced)]
#[kube(status = "DruidConnectionStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DruidConnectionSpec {
    /// The Druid to connect.
    pub druid: DruidConnectionDruid,
    /// The Superset to connect.
    pub superset: DruidConnectionSuperset,
}

/// The Druid to connect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidConnectionDruid {
    /// The name of the stacklet.
    pub name: String,
    /// The namespace. Defaults to the namespace of the `DruidConnection` if it is not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The Superset to connect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DruidConnectionSuperset {
    /// The name of the stacklet.
    pub name: String,
    /// The namespace. Defaults to the namespace of the `DruidConnection` if it is not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DruidConnectionStatus {
    pub condition: DruidConnectionStatusCondition,
    /// Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startedAt")]
    pub started_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DruidConnectionStatusCondition {
    Pending,
    Importing,
    Ready,
    Failed,
}

