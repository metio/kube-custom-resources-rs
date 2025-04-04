// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeedge/kubeedge/reliablesyncs.kubeedge.io/v1alpha1/clusterobjectsyncs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ObjectSyncSpec stores the details of objects that persist to the edge.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "reliablesyncs.kubeedge.io", version = "v1alpha1", kind = "ClusterObjectSync", plural = "clusterobjectsyncs")]
#[kube(status = "ClusterObjectSyncStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterObjectSyncSpec {
    /// ObjectAPIVersion is the APIVersion of the object
    /// that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectAPIVersion")]
    pub object_api_version: Option<String>,
    /// ObjectType is the kind of the object
    /// that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectKind")]
    pub object_kind: Option<String>,
    /// ObjectName is the name of the object
    /// that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectName")]
    pub object_name: Option<String>,
}

/// ObjectSyncStatus stores the resourceversion of objects that persist to the edge.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterObjectSyncStatus {
    /// ObjectResourceVersion is the resourceversion of the object
    /// that was successfully persist to the edge node.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectResourceVersion")]
    pub object_resource_version: Option<String>,
}

