// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/bpfman/bpfman-operator/bpfman.io/v1alpha1/bpfapplicationstates.yaml
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// BpfApplicationStateStatus reflects the status of the BpfApplication on the given node
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatus {
    /// appLoadStatus reflects the status of loading the bpf application on the
    /// given node.
    #[serde(rename = "appLoadStatus")]
    pub app_load_status: String,
    /// Conditions contains the overall status of the BpfApplicationState object
    /// on the given node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// node is the name of the node for this BpfApplicationStateSpec.
    pub node: String,
    /// programs is a list of bpf programs contained in the parent application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<BpfApplicationStateStatusPrograms>>,
    /// updateCount is the number of times the BpfApplicationState has been updated. Set to 1
    /// when the object is created, then it is incremented prior to each update.
    /// This allows us to verify that the API server has the updated object prior
    /// to starting a new Reconcile operation.
    #[serde(rename = "updateCount")]
    pub update_count: i64,
}

/// BpfApplicationProgramState defines the desired state of BpfApplication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BpfApplicationStateStatusPrograms {
    /// name is the name of the function that is the entry point for the BPF
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
    /// tc defines the desired state of the application's TcPrograms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tc: Option<BpfApplicationStateStatusProgramsTc>,
    /// tcx defines the desired state of the application's TcxPrograms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcx: Option<BpfApplicationStateStatusProgramsTcx>,
    /// type specifies the bpf program type
    #[serde(rename = "type")]
    pub r#type: BpfApplicationStateStatusProgramsType,
    /// uprobe defines the desired state of the application's UprobePrograms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uprobe: Option<BpfApplicationStateStatusProgramsUprobe>,
    /// uretprobe defines the desired state of the application's UretprobePrograms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uretprobe: Option<BpfApplicationStateStatusProgramsUretprobe>,
    /// xdp defines the desired state of the application's XdpPrograms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xdp: Option<BpfApplicationStateStatusProgramsXdp>,
}

/// tc defines the desired state of the application's TcPrograms.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTc {
    /// links is the List of attach points for the BPF program on the given node. Each entry
    /// in *AttachInfoState represents a specific, unique attach point that is
    /// derived from *AttachInfo by fully expanding any selectors.  Each entry
    /// also contains information about the attach point required by the
    /// reconciler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsTcLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcLinks {
    /// direction specifies the direction of traffic the tc program should
    /// attach to for a given network device.
    pub direction: BpfApplicationStateStatusProgramsTcLinksDirection,
    /// interfaceName is the Interface name to attach the tc program to.
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
    /// netnsPath is a path to a Network namespace to attach the tc program in.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority specifies the priority of the tc program in relation to
    /// other programs of the same type with the same attach point. It is a value
    /// from 0 to 1000 where lower values have higher precedence.
    pub priority: i32,
    /// proceedOn allows the user to call other tc programs in chain on this exit code.
    /// Multiple values are supported by repeating the parameter.
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

/// tcx defines the desired state of the application's TcxPrograms.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcx {
    /// links is the List of attach points for the BPF program on the given node. Each entry
    /// in *AttachInfoState represents a specific, unique attach point that is
    /// derived from *AttachInfo by fully expanding any selectors.  Each entry
    /// also contains information about the attach point required by the
    /// reconciler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsTcxLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsTcxLinks {
    /// direction specifies the direction of traffic the tcx program should
    /// attach to for a given network device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<BpfApplicationStateStatusProgramsTcxLinksDirection>,
    /// interfaceName is the Interface name to attach the tc program to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "interfaceName")]
    pub interface_name: Option<String>,
    /// linkId is an identifier for the link assigned by bpfman. This field is
    /// empty until the program is successfully attached and bpfman returns the
    /// id.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkId")]
    pub link_id: Option<i32>,
    /// linkStatus reflects whether the attachment has been reconciled
    /// successfully, and if not, why.
    #[serde(rename = "linkStatus")]
    pub link_status: String,
    /// netnsPath is the path to the Network namespace to attach the tcx program
    /// in.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority specifies the priority of the tcx program in relation to
    /// other programs of the same type with the same attach point. It is a value
    /// from 0 to 1000 where lower values have higher precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
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

/// BpfApplicationProgramState defines the desired state of BpfApplication
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

/// uprobe defines the desired state of the application's UprobePrograms.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUprobe {
    /// List of attach points for the BPF program on the given node. Each entry
    /// in *AttachInfoState represents a specific, unique attach point that is
    /// derived from *AttachInfo by fully expanding any selectors.  Each entry
    /// also contains information about the attach point required by the
    /// reconciler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsUprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUprobeLinks {
    /// containerPid is container pid to attach the uprobe program in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPid")]
    pub container_pid: Option<i32>,
    /// function to attach the uprobe to.
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
    /// offset added to the address of the function for uprobe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// pid is Only execute uprobe for given process identification number (PID). If PID
    /// is not provided, uprobe executes for all PIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// target is the library name or the absolute path to a binary or library.
    pub target: String,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// uretprobe defines the desired state of the application's UretprobePrograms.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUretprobe {
    /// List of attach points for the BPF program on the given node. Each entry
    /// in *AttachInfoState represents a specific, unique attach point that is
    /// derived from *AttachInfo by fully expanding any selectors.  Each entry
    /// also contains information about the attach point required by the
    /// reconciler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsUretprobeLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsUretprobeLinks {
    /// containerPid is container pid to attach the uprobe program in.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerPid")]
    pub container_pid: Option<i32>,
    /// function to attach the uprobe to.
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
    /// offset added to the address of the function for uprobe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// pid is Only execute uprobe for given process identification number (PID). If PID
    /// is not provided, uprobe executes for all PIDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// target is the library name or the absolute path to a binary or library.
    pub target: String,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

/// xdp defines the desired state of the application's XdpPrograms.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsXdp {
    /// links is the list of attach points for the BPF program on the given node. Each entry
    /// in *AttachInfoState represents a specific, unique attach point that is
    /// derived from *AttachInfo by fully expanding any selectors.  Each entry
    /// also contains information about the attach point required by the
    /// reconciler
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BpfApplicationStateStatusProgramsXdpLinks>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpfApplicationStateStatusProgramsXdpLinks {
    /// interfaceName is the interface name to attach the xdp program to.
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
    /// netnsPath is the path to the Network namespace to attach the xdp program
    /// in.
    #[serde(rename = "netnsPath")]
    pub netns_path: String,
    /// priority specifies the priority of the xdp program in relation to
    /// other programs of the same type with the same attach point. It is a value
    /// from 0 to 1000 where lower values have higher precedence.
    pub priority: i32,
    /// proceedOn allows the user to call other xdp programs in chain on this exit code.
    /// Multiple values are supported by repeating the parameter.
    #[serde(rename = "proceedOn")]
    pub proceed_on: Vec<String>,
    /// shouldAttach reflects whether the attachment should exist.
    #[serde(rename = "shouldAttach")]
    pub should_attach: bool,
    /// uuid is an Unique identifier for the attach point assigned by bpfman agent.
    pub uuid: String,
}

