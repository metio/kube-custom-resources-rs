// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/kernel-module-management/kmm.sigs.x-k8s.io/v1beta1/nodemodulesconfigs.yaml --derive=Default --derive=PartialEq
// kopium version: 0.17.2

use kube::CustomResource;
use serde::{Serialize, Deserialize};

/// NodeModulesConfigSpec describes the desired state of modules on the node
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kmm.sigs.x-k8s.io", version = "v1beta1", kind = "NodeModulesConfig", plural = "nodemodulesconfigs")]
#[kube(status = "NodeModulesConfigStatus")]
#[kube(schema = "disabled")]
pub struct NodeModulesConfigSpec {
    /// Modules list the spec of all the modules that need to be executed
    /// on the node
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<NodeModulesConfigModules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModules {
    pub config: NodeModulesConfigModulesConfig,
    /// LocalObjectReference contains enough information to let you locate the
    /// referenced object inside the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageRepoSecret")]
    pub image_repo_secret: Option<NodeModulesConfigModulesImageRepoSecret>,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModulesConfig {
    #[serde(rename = "containerImage")]
    pub container_image: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inTreeModuleToRemove")]
    pub in_tree_module_to_remove: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inTreeModulesToRemove")]
    pub in_tree_modules_to_remove: Option<Vec<String>>,
    /// When InsecurePull is true, the container image can be pulled without TLS.
    #[serde(rename = "insecurePull")]
    pub insecure_pull: bool,
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,
    pub modprobe: NodeModulesConfigModulesConfigModprobe,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModulesConfigModprobe {
    /// Args is an optional list of arguments to be passed to modprobe before the name of the kernel module.
    /// The resulting commands will be: `modprobe ${Args} module_name`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<NodeModulesConfigModulesConfigModprobeArgs>,
    /// DirName is the root directory for modules.
    /// It adds `-d ${DirName}` to the modprobe command-line.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dirName")]
    pub dir_name: Option<String>,
    /// FirmwarePath is the path of the firmware(s).
    /// The firmware(s) will be copied to the host for the kernel to find them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firmwarePath")]
    pub firmware_path: Option<String>,
    /// ModuleName is the name of the Module to be loaded.
    /// This field can only be unset if rawArgs is set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "moduleName")]
    pub module_name: Option<String>,
    /// ModulesLoadingOrder defines the dependency between kernel modules loading, in case
    /// it was not created by depmod (independent kernel modules).
    /// The list order should be: upmost module, then the module it depends on and so on.
    /// Example: if moduleA depends on first loading moduleB, and moduleB depends on first loading moduleC
    /// the entry should look:
    /// ModulesLoadingOrder:
    ///    - moduleA
    ///    - moduleB
    ///    - moduleC
    /// In order to load all 3 modules, moduleA shoud be defined in the ModuleName parameter of this struct
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modulesLoadingOrder")]
    pub modules_loading_order: Option<Vec<String>>,
    /// Parameters is an optional list of kernel module parameters to be provided to modprobe.
    /// They should be in the form of key=value and will be separated by spaces in the modprobe command.
    /// The resulting loading command will be: `modprobe module_name ${Parameters}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// If RawArgs are specified, they are passed straight to the modprobe binary; all other properties in this
    /// object are ignored.
    /// The resulting commands will be: `modprobe ${RawArgs}`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawArgs")]
    pub raw_args: Option<NodeModulesConfigModulesConfigModprobeRawArgs>,
}

/// Args is an optional list of arguments to be passed to modprobe before the name of the kernel module.
/// The resulting commands will be: `modprobe ${Args} module_name`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModulesConfigModprobeArgs {
    /// Load is an optional list of arguments to be used when loading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<Vec<String>>,
    /// Unload is an optional list of arguments to be used when unloading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unload: Option<Vec<String>>,
}

/// If RawArgs are specified, they are passed straight to the modprobe binary; all other properties in this
/// object are ignored.
/// The resulting commands will be: `modprobe ${RawArgs}`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModulesConfigModprobeRawArgs {
    /// Load is an optional list of arguments to be used when loading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<Vec<String>>,
    /// Unload is an optional list of arguments to be used when unloading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unload: Option<Vec<String>>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigModulesImageRepoSecret {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NodeModuleConfigStatus is the most recently observed status of the KMM modules on node.
/// It is populated by the system and is read-only.
/// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatus {
    /// Modules contain observations about each Module's node state status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<NodeModulesConfigStatusModules>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<NodeModulesConfigStatusModulesConfig>,
    /// LocalObjectReference contains enough information to let you locate the
    /// referenced object inside the same namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imageRepoSecret")]
    pub image_repo_secret: Option<NodeModulesConfigStatusModulesImageRepoSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "serviceAccountName")]
    pub service_account_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModulesConfig {
    #[serde(rename = "containerImage")]
    pub container_image: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inTreeModuleToRemove")]
    pub in_tree_module_to_remove: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inTreeModulesToRemove")]
    pub in_tree_modules_to_remove: Option<Vec<String>>,
    /// When InsecurePull is true, the container image can be pulled without TLS.
    #[serde(rename = "insecurePull")]
    pub insecure_pull: bool,
    #[serde(rename = "kernelVersion")]
    pub kernel_version: String,
    pub modprobe: NodeModulesConfigStatusModulesConfigModprobe,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModulesConfigModprobe {
    /// Args is an optional list of arguments to be passed to modprobe before the name of the kernel module.
    /// The resulting commands will be: `modprobe ${Args} module_name`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<NodeModulesConfigStatusModulesConfigModprobeArgs>,
    /// DirName is the root directory for modules.
    /// It adds `-d ${DirName}` to the modprobe command-line.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dirName")]
    pub dir_name: Option<String>,
    /// FirmwarePath is the path of the firmware(s).
    /// The firmware(s) will be copied to the host for the kernel to find them.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firmwarePath")]
    pub firmware_path: Option<String>,
    /// ModuleName is the name of the Module to be loaded.
    /// This field can only be unset if rawArgs is set.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "moduleName")]
    pub module_name: Option<String>,
    /// ModulesLoadingOrder defines the dependency between kernel modules loading, in case
    /// it was not created by depmod (independent kernel modules).
    /// The list order should be: upmost module, then the module it depends on and so on.
    /// Example: if moduleA depends on first loading moduleB, and moduleB depends on first loading moduleC
    /// the entry should look:
    /// ModulesLoadingOrder:
    ///    - moduleA
    ///    - moduleB
    ///    - moduleC
    /// In order to load all 3 modules, moduleA shoud be defined in the ModuleName parameter of this struct
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modulesLoadingOrder")]
    pub modules_loading_order: Option<Vec<String>>,
    /// Parameters is an optional list of kernel module parameters to be provided to modprobe.
    /// They should be in the form of key=value and will be separated by spaces in the modprobe command.
    /// The resulting loading command will be: `modprobe module_name ${Parameters}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// If RawArgs are specified, they are passed straight to the modprobe binary; all other properties in this
    /// object are ignored.
    /// The resulting commands will be: `modprobe ${RawArgs}`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawArgs")]
    pub raw_args: Option<NodeModulesConfigStatusModulesConfigModprobeRawArgs>,
}

/// Args is an optional list of arguments to be passed to modprobe before the name of the kernel module.
/// The resulting commands will be: `modprobe ${Args} module_name`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModulesConfigModprobeArgs {
    /// Load is an optional list of arguments to be used when loading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<Vec<String>>,
    /// Unload is an optional list of arguments to be used when unloading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unload: Option<Vec<String>>,
}

/// If RawArgs are specified, they are passed straight to the modprobe binary; all other properties in this
/// object are ignored.
/// The resulting commands will be: `modprobe ${RawArgs}`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModulesConfigModprobeRawArgs {
    /// Load is an optional list of arguments to be used when loading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load: Option<Vec<String>>,
    /// Unload is an optional list of arguments to be used when unloading the kernel module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unload: Option<Vec<String>>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeModulesConfigStatusModulesImageRepoSecret {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
