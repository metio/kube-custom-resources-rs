// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/volumeattachments.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// VolumeAttachmentSpec defines the desired state of Longhorn VolumeAttachment
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "VolumeAttachment", plural = "volumeattachments")]
#[kube(namespaced)]
#[kube(status = "VolumeAttachmentStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VolumeAttachmentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attachmentTickets")]
    pub attachment_tickets: Option<BTreeMap<String, VolumeAttachmentAttachmentTickets>>,
    /// The name of Longhorn volume of this VolumeAttachment
    pub volume: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentAttachmentTickets {
    /// A sequence number representing a specific generation of the desired state.
    /// Populated by the system. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    /// The unique ID of this attachment. Used to differentiate different attachments of the same volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The node that this attachment is requesting
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    /// Optional additional parameter for this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// VolumeAttachmentStatus defines the observed state of Longhorn VolumeAttachment
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attachmentTicketStatuses")]
    pub attachment_ticket_statuses: Option<BTreeMap<String, VolumeAttachmentStatusAttachmentTicketStatuses>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VolumeAttachmentStatusAttachmentTicketStatuses {
    /// Record any error when trying to fulfill this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A sequence number representing a specific generation of the desired state.
    /// Populated by the system. Read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    /// The unique ID of this attachment. Used to differentiate different attachments of the same volume.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicate whether this attachment ticket has been satisfied
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub satisfied: Option<bool>,
}

