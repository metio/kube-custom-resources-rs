// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/bpfman/bpfman-operator/bpfman.io/v1alpha1/bpfapplicationstates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// status reflects the status of a BpfApplication instance for the given node.
/// appLoadStatus and conditions provide an overall status for the given node,
/// while each item in the programs list provides a per eBPF program status for
/// the given node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatus {
    /// appLoadStatus reflects the status of loading the eBPF application on the
    /// given node.
    /// 
    /// 
    /// NotLoaded is a temporary state that is assigned when a
    /// ClusterBpfApplicationState is created and the initial reconcile is being
    /// processed.
    /// 
    /// 
    /// LoadSuccess is returned if all the programs have been loaded with no
    /// errors.
    /// 
    /// 
    /// LoadError is returned if one or more programs encountered an error and
    /// were not loaded.
    /// 
    /// 
    /// NotSelected is returned if this application did not select to run on this
    /// Kubernetes node.
    /// 
    /// 
    /// UnloadSuccess is returned when all the programs were successfully
    /// unloaded.
    /// 
    /// 
    /// UnloadError is returned if one or more programs encountered an error when
    /// being unloaded.
    #[serde(rename = "appLoadStatus")]
    pub app_load_status: String,
    /// conditions contains the summary state of the BpfApplication for the given
    /// Kubernetes node. If one or more programs failed to load or attach to the
    /// designated attachment point, the condition will report the error. If more
    /// than one error has occurred, condition will contain the first error
    /// encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// node is the name of the Kubernets node for this BpfApplicationState.
    pub node: String,
    /// programs is a list of eBPF programs contained in the parent BpfApplication
    /// instance. Each entry in the list contains the derived program attributes as
    /// well as the attach status for each program on the given Kubernetes node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<BpfApplicationStateStatusPrograms>>,
    /// UpdateCount tracks the number of times the BpfApplicationState object has
    /// been updated. The bpfman agent initializes it to 1 when it creates the
    /// object, and then increments it before each subsequent update. It serves
    /// as a lightweight sequence number to verify that the API server is serving
    /// the most recent version of the object before beginning a new Reconcile
    /// operation.
    #[serde(rename = "updateCount")]
    pub update_count: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BpfApplicationStateStatusPrograms {
    /// name is the name of the function that is the entry point for the eBPF
    /// program
    pub name: String,
    /// programId is the id of the program in the kernel.  Not set until the
    /// program is loaded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "programId")]
    pub program_id: Option<i32>,
    /// programLinkStatus reflects whether all links requested for the program
    /// are in the correct state.
    #[serde(rename = "programLinkStatus")]
    pub program_link_status: String,
    /// tc contains the attachment data for a TC program when type is set to TC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tc: Option<BpfApplicationStateStatusProgramsTc>,
    /// tcx contains the attachment data for a TCX program when type is set to TCX.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcx: Option<BpfApplicationStateStatusProgramsTcx>,
    /// type specifies the provisioned eBPF program type for this program entry.
    /// Type will be one of:
    ///   TC, TCX, UProbe, URetProbe, XDP
    /// 
    /// 
    /// When set to TC, the tc object will be populated with the eBPF program data
    /// associated with a TC program.
    /// 
    /// 
    /// When set to TCX, the tcx object will be populated with the eBPF program
    /// data associated with a TCX program.
    /// 
    /// 
    /// When set to UProbe, the uprobe object will be populated with the eBPF
    /// program data associated with a UProbe program.
    /// 
    /// 
    /// When set to URetProbe, the uretprobe object will be populated with the eBPF
    /// program data associated with a URetProbe program.
    /// 
    /// 
    /// When set to XDP, the xdp object will be populated with the eBPF program data
    /// associated with a URetProbe program.
    #[serde(rename = "type")]
    pub r#type: BpfApplicationStateStatusProgramsType,
    /// uprobe contains the attachment data for a UProbe program when type is set to
    /// UProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uprobe: Option<BpfApplicationStateStatusProgramsUprobe>,
    /// uretprobe contains the attachment data for a URetProbe program when type is
    /// set to URetProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uretprobe: Option<BpfApplicationStateStatusProgramsUretprobe>,
    /// xdp contains the attachment data for an XDP program when type is set to XDP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xdp: Option<BpfApplicationStateStatusProgramsXdp>,
}

/// tc contains the attachment data for a TC program when type is set to TC.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTc {
    /// links is a list of attachment points for the TC program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsTcLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcLinks {
    /// direction is the provisioned direction of traffic, Ingress or Egress, the TC
    /// program should be attached for a given network device.
    pub direction: BpfApplicationStateStatusProgramsTcLinksDirection,
    /// interfaceName is the name of the interface the TC program should be
    /// attached.
    #[serde(rename = "interfaceName")]
    pub interface_name: String,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// netnsPath is the path to the network namespace inside of which the TC
    /// program should be attached.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority is the provisioned priority of the TC program in relation to other
    /// programs of the same type with the same attach point. It is a value from 0
    /// to 1000, where lower values have higher precedence.
    pub priority: i32,
    /// proceedOn is the provisioned list of proceedOn values. proceedOn allows the
    /// user to call other TC programs in a chain, or not call the next program in a
    /// chain based on the exit code of a TC program .Multiple values are supported.
    #[serde(rename = "proceedOn")]
    pub proceed_on: Vec<String>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BpfApplicationStateStatusProgramsTcLinksDirection {
    Ingress,
    Egress,
}

/// tcx contains the attachment data for a TCX program when type is set to TCX.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcx {
    /// links is a list of attachment points for the TCX program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsTcxLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcxLinks {
    /// direction is the provisioned direction of traffic, Ingress or Egress, the
    /// TCX program should be attached for a given network device.
    pub direction: BpfApplicationStateStatusProgramsTcxLinksDirection,
    /// interfaceName is the name of the interface the TCX program should be
    /// attached.
    #[serde(rename = "interfaceName")]
    pub interface_name: String,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// netnsPath is the path to the network namespace inside of which the TCX
    /// program should be attached.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority is the provisioned priority of the TCX program in relation to other
    /// programs of the same type with the same attach point. It is a value from 0
    /// to 1000, where lower values have higher precedence.
    pub priority: i32,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BpfApplicationStateStatusProgramsTcxLinksDirection {
    Ingress,
    Egress,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BpfApplicationStateStatusProgramsType {
    #[serde(rename = "XDP")]
    Xdp,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TCX")]
    Tcx,
    UProbe,
    URetProbe,
}

/// uprobe contains the attachment data for a UProbe program when type is set to
/// UProbe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUprobe {
    /// links is a list of attachment points for the UProbe program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsUprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUprobeLinks {
    /// If containers is provisioned in the BpfApplication instance, containerPid is
    /// the derived PID of the container the UProbe or URetProbe this attachment
    /// point is attached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPid")]
    pub container_pid: Option<i32>,
    /// function is the provisioned name of the user-space function the UProbe
    /// program should be attached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// offset is the provisioned offset, whose value is added to the address of the
    /// attachment point function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// pid is the provisioned pid. If set, pid limits the execution of the UProbe
    /// or URetProbe to the provided process identification number (PID). If pid is
    /// not provided, the UProbe or URetProbe executes for all PIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// target is the provisioned user-space library name or the absolute path to a
    /// binary or library.
    pub target: String,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// uretprobe contains the attachment data for a URetProbe program when type is
/// set to URetProbe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUretprobe {
    /// links is a list of attachment points for the UProbe program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsUretprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUretprobeLinks {
    /// If containers is provisioned in the BpfApplication instance, containerPid is
    /// the derived PID of the container the UProbe or URetProbe this attachment
    /// point is attached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPid")]
    pub container_pid: Option<i32>,
    /// function is the provisioned name of the user-space function the UProbe
    /// program should be attached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// offset is the provisioned offset, whose value is added to the address of the
    /// attachment point function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// pid is the provisioned pid. If set, pid limits the execution of the UProbe
    /// or URetProbe to the provided process identification number (PID). If pid is
    /// not provided, the UProbe or URetProbe executes for all PIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// target is the provisioned user-space library name or the absolute path to a
    /// binary or library.
    pub target: String,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// xdp contains the attachment data for an XDP program when type is set to XDP.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsXdp {
    /// links is a list of attachment points for the XDP program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsXdpLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsXdpLinks {
    /// interfaceName is the name of the interface the XDP program should be
    /// attached.
    #[serde(rename = "interfaceName")]
    pub interface_name: String,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// netnsPath is the path to the network namespace inside of which the XDP
    /// program should be attached.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority is the provisioned priority of the XDP program in relation to other
    /// programs of the same type with the same attach point. It is a value from 0
    /// to 1000, where lower values have higher precedence.
    pub priority: i32,
    /// proceedOn is the provisioned list of proceedOn values. proceedOn allows the
    /// user to call other TC programs in a chain, or not call the next program in a
    /// chain based on the exit code of a TC program .Multiple values are supported.
    #[serde(rename = "proceedOn")]
    pub proceed_on: Vec<String>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

