// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/bpfman/bpfman-operator/bpfman.io/v1alpha1/clusterbpfapplicationstates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// status reflects the status of a ClusterBpfApplication instance for the given
/// node. appLoadStatus and conditions provide an overall status for the given
/// node, while each item in the programs list provides a per eBPF program
/// status for the given node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatus {
    /// appLoadStatus reflects the status of loading the eBPF application on the
    /// given node.
    /// 
    /// 
    /// NotLoaded is a temporary state that is assigned when a
    /// ClusterBpfApplicationState is created and the initial reconcile is being
    /// processed.
    /// 
    /// 
    /// LoadSuccess is returned if all the programs have been loaded with no errors.
    /// 
    /// 
    /// LoadError is returned if one or more programs encountered an error and were
    /// not loaded.
    /// 
    /// 
    /// NotSelected is returned if this application did not select to run on this
    /// Kubernetes node.
    /// 
    /// 
    /// UnloadSuccess is returned when all the programs were successfully unloaded.
    /// 
    /// 
    /// UnloadError is returned if one or more programs encountered an error when
    /// being unloaded.
    #[serde(rename = "appLoadStatus")]
    pub app_load_status: String,
    /// conditions contains the summary state of the ClusterBpfApplication for the
    /// given Kubernetes node. If one or more programs failed to load or attach to
    /// the designated attachment point, the condition will report the error. If
    /// more than one error has occurred, condition will contain the first error
    /// encountered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// node is the name of the Kubernetes node for this ClusterBpfApplicationState.
    pub node: String,
    /// programs is a list of eBPF programs contained in the parent
    /// ClusterBpfApplication instance. Each entry in the list contains the derived
    /// program attributes as well as the attach status for each program on the
    /// given Kubernetes node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<ClusterBpfApplicationStateStatusPrograms>>,
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
pub struct ClusterBpfApplicationStateStatusPrograms {
    /// fentry contains the attachment data for an FEntry program when type is set
    /// to FEntry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fentry: Option<ClusterBpfApplicationStateStatusProgramsFentry>,
    /// fexit contains the attachment data for an FExit program when type is set to
    /// FExit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fexit: Option<ClusterBpfApplicationStateStatusProgramsFexit>,
    /// kprobe contains the attachment data for a KProbe program when type is set to
    /// KProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kprobe: Option<ClusterBpfApplicationStateStatusProgramsKprobe>,
    /// kretprobe contains the attachment data for a KRetProbe program when type is
    /// set to KRetProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kretprobe: Option<ClusterBpfApplicationStateStatusProgramsKretprobe>,
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
    pub tc: Option<ClusterBpfApplicationStateStatusProgramsTc>,
    /// tcx contains the attachment data for a TCX program when type is set to TCX.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcx: Option<ClusterBpfApplicationStateStatusProgramsTcx>,
    /// tracepoint contains the attachment data for a Tracepoint program when type
    /// is set to Tracepoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracepoint: Option<ClusterBpfApplicationStateStatusProgramsTracepoint>,
    /// type specifies the provisioned eBPF program type for this program entry.
    /// Type will be one of:
    ///   FEntry, FExit, KProbe, KRetProbe, TC, TCX, Tracepoint, UProbe,
    ///   URetProbe, XDP
    /// 
    /// 
    /// When set to FEntry, the fentry object will be populated with the eBPF
    /// program data associated with an FEntry program.
    /// 
    /// 
    /// When set to FExit, the fexit object will be populated with the eBPF program
    /// data associated with an FExit program.
    /// 
    /// 
    /// When set to KProbe, the kprobe object will be populated with the eBPF
    /// program data associated with a KProbe program.
    /// 
    /// 
    /// When set to KRetProbe, the kretprobe object will be populated with the
    /// eBPF program data associated with a KRetProbe program.
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
    /// When set to Tracepoint, the tracepoint object will be populated with the
    /// eBPF program data associated with a Tracepoint program.
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
    pub r#type: ClusterBpfApplicationStateStatusProgramsType,
    /// uprobe contains the attachment data for a UProbe program when type is set to
    /// UProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uprobe: Option<ClusterBpfApplicationStateStatusProgramsUprobe>,
    /// uretprobe contains the attachment data for a URetProbe program when type is
    /// set to URetProbe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uretprobe: Option<ClusterBpfApplicationStateStatusProgramsUretprobe>,
    /// xdp contains the attachment data for an XDP program when type is set to XDP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xdp: Option<ClusterBpfApplicationStateStatusProgramsXdp>,
}

/// fentry contains the attachment data for an FEntry program when type is set
/// to FEntry.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsFentry {
    /// function is a required field and specifies the name of the Linux kernel
    /// function to attach the FEntry program. function must not be an empty string,
    /// must not exceed 64 characters in length, must start with alpha characters
    /// and must only contain alphanumeric characters.
    pub function: String,
    /// links is a list of attachment points for the FEntry program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsFentryLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsFentryLinks {
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// fexit contains the attachment data for an FExit program when type is set to
/// FExit.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsFexit {
    /// function is a required field and specifies the name of the Linux kernel
    /// function to attach the FExit program. function must not be an empty string,
    /// must not exceed 64 characters in length, must start with alpha characters
    /// and must only contain alphanumeric characters.
    pub function: String,
    /// links is a list of attachment points for the FExit program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsFexitLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsFexitLinks {
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// kprobe contains the attachment data for a KProbe program when type is set to
/// KProbe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsKprobe {
    /// links is a list of attachment points for the KProbe program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsKprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsKprobeLinks {
    /// function is the provisioned name of the Linux kernel function the KProbe
    /// program should be attached.
    pub function: String,
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
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// kretprobe contains the attachment data for a KRetProbe program when type is
/// set to KRetProbe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsKretprobe {
    /// links is a list of attachment points for the KRetProbe program. Each entry
    /// in the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsKretprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsKretprobeLinks {
    /// function is the provisioned name of the Linux kernel function the KRetProbe
    /// program should be attached.
    pub function: String,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// tc contains the attachment data for a TC program when type is set to TC.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTc {
    /// links is a list of attachment points for the TC program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsTcLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTcLinks {
    /// direction is the provisioned direction of traffic, Ingress or Egress, the TC
    /// program should be attached for a given network device.
    pub direction: ClusterBpfApplicationStateStatusProgramsTcLinksDirection,
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
    /// netnsPath is the optional path to the network namespace inside of which the
    /// TC program should be attached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "netnsPath")]
    pub netns_path: Option<String>,
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
pub enum ClusterBpfApplicationStateStatusProgramsTcLinksDirection {
    Ingress,
    Egress,
}

/// tcx contains the attachment data for a TCX program when type is set to TCX.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTcx {
    /// links is a list of attachment points for the TCX program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsTcxLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTcxLinks {
    /// direction is the provisioned direction of traffic, Ingress or Egress, the TC
    /// program should be attached for a given network device.
    pub direction: ClusterBpfApplicationStateStatusProgramsTcxLinksDirection,
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
    /// netnsPath is the optional path to the network namespace inside of which the
    /// TCX program should be attached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "netnsPath")]
    pub netns_path: Option<String>,
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
pub enum ClusterBpfApplicationStateStatusProgramsTcxLinksDirection {
    Ingress,
    Egress,
}

/// tracepoint contains the attachment data for a Tracepoint program when type
/// is set to Tracepoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTracepoint {
    /// links is a list of attachment points for the Tracepoint program. Each entry
    /// in the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsTracepointLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsTracepointLinks {
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// The name of a kernel tracepoint to attach the bpf program to.
    /// name is the provisioned name of the Linux kernel tracepoint function the
    /// Tracepoint program should be attached.
    pub name: String,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterBpfApplicationStateStatusProgramsType {
    FEntry,
    FExit,
    KProbe,
    KRetProbe,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TCX")]
    Tcx,
    TracePoint,
    UProbe,
    URetProbe,
    #[serde(rename = "XDP")]
    Xdp,
}

/// uprobe contains the attachment data for a UProbe program when type is set to
/// UProbe.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsUprobe {
    /// links is a list of attachment points for the UProbe program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsUprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsUprobeLinks {
    /// If containers is provisioned in the ClusterBpfApplication instance,
    /// containerPid is the derived PID of the container the UProbe or URetProbe this
    /// attachment point is attached.
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
pub struct ClusterBpfApplicationStateStatusProgramsUretprobe {
    /// links is a list of attachment points for the UProbe program. Each entry in
    /// the list includes a linkStatus, which indicates if the attachment was
    /// successful or not on this node, a linkId, which is the kernel ID for the
    /// link if successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsUretprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsUretprobeLinks {
    /// If containers is provisioned in the ClusterBpfApplication instance,
    /// containerPid is the derived PID of the container the UProbe or URetProbe this
    /// attachment point is attached.
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
pub struct ClusterBpfApplicationStateStatusProgramsXdp {
    /// links is a list of attachment points for the XDP program. Each entry in the
    /// list includes a linkStatus, which indicates if the attachment was successful
    /// or not on this node, a linkId, which is the kernel ID for the link if
    /// successfully attached, and other attachment specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClusterBpfApplicationStateStatusProgramsXdpLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterBpfApplicationStateStatusProgramsXdpLinks {
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
    /// netnsPath is the optional path to the network namespace inside of which the
    /// XDP program should be attached.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "netnsPath")]
    pub netns_path: Option<String>,
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

