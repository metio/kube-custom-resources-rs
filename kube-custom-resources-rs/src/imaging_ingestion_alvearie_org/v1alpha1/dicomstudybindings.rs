// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/Alvearie/imaging-ingestion/imaging-ingestion.alvearie.org/v1alpha1/dicomstudybindings.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DicomStudyBindingSpec defines the desired state of DicomStudyBinding
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "imaging-ingestion.alvearie.org", version = "v1alpha1", kind = "DicomStudyBinding", plural = "dicomstudybindings")]
#[kube(namespaced)]
#[kube(status = "DicomStudyBindingStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DicomStudyBindingSpec {
    /// Binding Config Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindingConfigName")]
    pub binding_config_name: Option<String>,
    /// Binding Secret Name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bindingSecretName")]
    pub binding_secret_name: Option<String>,
    /// DICOM Event Driven Ingestion Name
    #[serde(rename = "dicomEventDrivenIngestionName")]
    pub dicom_event_driven_ingestion_name: String,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// Image Pull Secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<DicomStudyBindingImagePullSecrets>>,
    /// Study Binding Spec
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "studyBinding")]
    pub study_binding: Option<DicomStudyBindingStudyBinding>,
}

/// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomStudyBindingImagePullSecrets {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Study Binding Spec
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomStudyBindingStudyBinding {
    /// Container Concurrency
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    /// Image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Max Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxReplicas")]
    pub max_replicas: Option<i32>,
    /// Min Replicas
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicas")]
    pub min_replicas: Option<i32>,
}

/// DicomStudyBindingStatus defines the observed state of DicomStudyBinding
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DicomStudyBindingStatus {
    /// Human-readable message indicating details about current operator phase or error
    pub message: String,
    /// Current phase of the operator
    pub phase: String,
    /// True if all resources are in a ready state and all work is done
    pub ready: bool,
    /// A map of all the secondary resources types and names created for this CR. e.g "Deployment": [ "DeploymentName1", "DeploymentName2" ]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secondaryResources")]
    pub secondary_resources: Option<BTreeMap<String, String>>,
}

