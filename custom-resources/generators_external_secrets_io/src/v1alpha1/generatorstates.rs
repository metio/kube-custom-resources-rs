// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/external-secrets/external-secrets/generators.external-secrets.io/v1alpha1/generatorstates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "generators.external-secrets.io", version = "v1alpha1", kind = "GeneratorState", plural = "generatorstates")]
#[kube(namespaced)]
#[kube(status = "GeneratorStateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct GeneratorStateSpec {
    /// GarbageCollectionDeadline is the time after which the generator state
    /// will be deleted.
    /// It is set by the controller which creates the generator state and
    /// can be set configured by the user.
    /// If the garbage collection deadline is not set the generator state will not be deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "garbageCollectionDeadline")]
    pub garbage_collection_deadline: Option<String>,
    /// Resource is the generator manifest that produced the state.
    /// It is a snapshot of the generator manifest at the time the state was produced.
    /// This manifest will be used to delete the resource. Any configuration that is referenced
    /// in the manifest should be available at the time of garbage collection. If that is not the case deletion will
    /// be blocked by a finalizer.
    pub resource: serde_json::Value,
    /// State is the state that was produced by the generator implementation.
    pub state: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GeneratorStateStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

