// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/ktsstudio/mirrors/mirrors.kts.studio/v1alpha1/secretmirrors.yaml --derive=Default --derive=PartialEq
// kopium version: 0.16.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// SecretMirrorSpec defines the desired state of SecretMirror
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "mirrors.kts.studio", version = "v1alpha1", kind = "SecretMirror", plural = "secretmirrors")]
#[kube(namespaced)]
#[kube(status = "SecretMirrorStatus")]
#[kube(schema = "disabled")]
pub struct SecretMirrorSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<SecretMirrorDestination>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollPeriodSeconds")]
    pub poll_period_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SecretMirrorSource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceRegex")]
    pub namespace_regex: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SecretMirrorStatus defines the observed state of SecretMirror
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SecretMirrorStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncTime")]
    pub last_sync_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mirrorStatus")]
    pub mirror_status: Option<SecretMirrorStatusMirrorStatus>,
}

/// SecretMirrorStatus defines the observed state of SecretMirror
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SecretMirrorStatusMirrorStatus {
    Pending,
    Active,
    Error,
}
