// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubeedge/kubeedge/devices.kubeedge.io/v1alpha2/devices.yaml --derive=Default --derive=PartialEq
// kopium version: 0.19.0

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// DeviceSpec represents a single device instance. It is an instantation of a device model.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "devices.kubeedge.io", version = "v1alpha2", kind = "Device", plural = "devices")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DeviceSpec {
    /// Data section describe a list of time-series properties which should be processed on edge node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<DeviceData>,
    /// Required: DeviceModelRef is reference to the device model used as a template to create the device instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceModelRef")]
    pub device_model_ref: Option<DeviceDeviceModelRef>,
    /// NodeSelector indicates the binding preferences between devices and nodes. Refer to k8s.io/kubernetes/pkg/apis/core NodeSelector for more details
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelector")]
    pub node_selector: Option<DeviceNodeSelector>,
    /// List of property visitors which describe how to access the device properties. PropertyVisitors must unique by propertyVisitor.propertyName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertyVisitors")]
    pub property_visitors: Option<Vec<DevicePropertyVisitors>>,
    /// Required: The protocol configuration used to connect to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<DeviceProtocol>,
}

/// Data section describe a list of time-series properties which should be processed on edge node.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceData {
    /// Required: A list of data properties, which are not required to be processed by edgecore
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataProperties")]
    pub data_properties: Option<Vec<DeviceDataDataProperties>>,
    /// Topic used by mapper, all data collected from dataProperties should be published to this topic, the default value is $ke/events/device/+/data/update
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataTopic")]
    pub data_topic: Option<String>,
}

/// DataProperty represents the device property for external use.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDataDataProperties {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The property name for which should be processed by external apps. This property should be present in the device model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertyName")]
    pub property_name: Option<String>,
}

/// Required: DeviceModelRef is reference to the device model used as a template to create the device instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceDeviceModelRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// NodeSelector indicates the binding preferences between devices and nodes. Refer to k8s.io/kubernetes/pkg/apis/core NodeSelector for more details
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceNodeSelector {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<DeviceNodeSelectorNodeSelectorTerms>,
}

/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceNodeSelectorNodeSelectorTerms {
    /// A list of node selector requirements by node's labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<DeviceNodeSelectorNodeSelectorTermsMatchExpressions>>,
    /// A list of node selector requirements by node's fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<DeviceNodeSelectorNodeSelectorTermsMatchFields>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceNodeSelectorNodeSelectorTermsMatchExpressions {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceNodeSelectorNodeSelectorTermsMatchFields {
    /// The label key that the selector applies to.
    pub key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    pub operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// DevicePropertyVisitor describes the specifics of accessing a particular device property. Visitors are intended to be consumed by device mappers which connect to devices and collect data / perform actions on the device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitors {
    /// Bluetooth represents a set of additional visitor config fields of bluetooth protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<DevicePropertyVisitorsBluetooth>,
    /// Define how frequent mapper will collect from device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectCycle")]
    pub collect_cycle: Option<i64>,
    /// CustomizedProtocol represents a set of visitor config fields of bluetooth protocol.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customizedProtocol")]
    pub customized_protocol: Option<DevicePropertyVisitorsCustomizedProtocol>,
    /// Customized values for visitor of provided protocols
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customizedValues")]
    pub customized_values: Option<BTreeMap<String, serde_json::Value>>,
    /// Modbus represents a set of additional visitor config fields of modbus protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modbus: Option<DevicePropertyVisitorsModbus>,
    /// Opcua represents a set of additional visitor config fields of opc-ua protocol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opcua: Option<DevicePropertyVisitorsOpcua>,
    /// Required: The device property name to be accessed. This should refer to one of the device properties defined in the device model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertyName")]
    pub property_name: Option<String>,
    /// Define how frequent mapper will report the value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reportCycle")]
    pub report_cycle: Option<i64>,
}

/// Bluetooth represents a set of additional visitor config fields of bluetooth protocol.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsBluetooth {
    /// Required: Unique ID of the corresponding operation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "characteristicUUID")]
    pub characteristic_uuid: Option<String>,
    /// Responsible for converting the data being read from the bluetooth device into a form that is understandable by the platform
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataConverter")]
    pub data_converter: Option<DevicePropertyVisitorsBluetoothDataConverter>,
    /// Responsible for converting the data coming from the platform into a form that is understood by the bluetooth device For example: "ON":[1], "OFF":[0]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataWrite")]
    pub data_write: Option<BTreeMap<String, String>>,
}

/// Responsible for converting the data being read from the bluetooth device into a form that is understandable by the platform
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsBluetoothDataConverter {
    /// Required: Specifies the end index of incoming byte stream to be considered to convert the data the value specified should be inclusive for example if 3 is specified it includes the third index
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endIndex")]
    pub end_index: Option<i64>,
    /// Specifies in what order the operations(which are required to be performed to convert incoming data into understandable form) are performed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orderOfOperations")]
    pub order_of_operations: Option<Vec<DevicePropertyVisitorsBluetoothDataConverterOrderOfOperations>>,
    /// Refers to the number of bits to shift left, if left-shift operation is necessary for conversion
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shiftLeft")]
    pub shift_left: Option<i64>,
    /// Refers to the number of bits to shift right, if right-shift operation is necessary for conversion
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shiftRight")]
    pub shift_right: Option<i64>,
    /// Required: Specifies the start index of the incoming byte stream to be considered to convert the data. For example: start-index:2, end-index:3 concatenates the value present at second and third index of the incoming byte stream. If we want to reverse the order we can give it as start-index:3, end-index:2
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startIndex")]
    pub start_index: Option<i64>,
}

/// Specify the operation that should be performed to convert incoming data into understandable form
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsBluetoothDataConverterOrderOfOperations {
    /// Required: Specifies the operation to be performed to convert incoming data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationType")]
    pub operation_type: Option<String>,
    /// Required: Specifies with what value the operation is to be performed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "operationValue")]
    pub operation_value: Option<f64>,
}

/// CustomizedProtocol represents a set of visitor config fields of bluetooth protocol.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsCustomizedProtocol {
    /// Required: The configData of customized protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configData")]
    pub config_data: Option<BTreeMap<String, serde_json::Value>>,
    /// Required: name of customized protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolName")]
    pub protocol_name: Option<String>,
}

/// Modbus represents a set of additional visitor config fields of modbus protocol.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsModbus {
    /// Indicates whether the high and low register swapped. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isRegisterSwap")]
    pub is_register_swap: Option<bool>,
    /// Indicates whether the high and low byte swapped. Defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isSwap")]
    pub is_swap: Option<bool>,
    /// Required: Limit number of registers to read/write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Required: Offset indicates the starting register number to read/write data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Required: Type of register
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register: Option<DevicePropertyVisitorsModbusRegister>,
    /// The scale to convert raw property data into final units. Defaults to 1.0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

/// Modbus represents a set of additional visitor config fields of modbus protocol.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DevicePropertyVisitorsModbusRegister {
    CoilRegister,
    DiscreteInputRegister,
    InputRegister,
    HoldingRegister,
}

/// Opcua represents a set of additional visitor config fields of opc-ua protocol.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DevicePropertyVisitorsOpcua {
    /// The name of opc-ua node
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "browseName")]
    pub browse_name: Option<String>,
    /// Required: The ID of opc-ua node, e.g. "ns=1,i=1005"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
}

/// Required: The protocol configuration used to connect to the device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocol {
    /// Protocol configuration for bluetooth
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bluetooth: Option<DeviceProtocolBluetooth>,
    /// Configuration for protocol common part
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<DeviceProtocolCommon>,
    /// Configuration for customized protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customizedProtocol")]
    pub customized_protocol: Option<DeviceProtocolCustomizedProtocol>,
    /// Protocol configuration for modbus
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modbus: Option<DeviceProtocolModbus>,
    /// Protocol configuration for opc-ua
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opcua: Option<DeviceProtocolOpcua>,
}

/// Protocol configuration for bluetooth
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolBluetooth {
    /// Unique identifier assigned to the device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "macAddress")]
    pub mac_address: Option<String>,
}

/// Configuration for protocol common part
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolCommon {
    /// Define retry times of mapper will collect from device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectRetryTimes")]
    pub collect_retry_times: Option<i64>,
    /// Define timeout of mapper collect from device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectTimeout")]
    pub collect_timeout: Option<i64>,
    /// Define collect type, sync or async.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collectType")]
    pub collect_type: Option<DeviceProtocolCommonCollectType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub com: Option<DeviceProtocolCommonCom>,
    /// Communication type, like tcp client, tcp server or COM
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "commType")]
    pub comm_type: Option<String>,
    /// Customized values for provided protocol
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customizedValues")]
    pub customized_values: Option<BTreeMap<String, serde_json::Value>>,
    /// Reconnecting retry times
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconnRetryTimes")]
    pub reconn_retry_times: Option<i64>,
    /// Reconnection timeout
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconnTimeout")]
    pub reconn_timeout: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<DeviceProtocolCommonTcp>,
}

/// Configuration for protocol common part
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceProtocolCommonCollectType {
    #[serde(rename = "sync")]
    Sync,
    #[serde(rename = "async")]
    Async,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolCommonCom {
    /// Required. BaudRate 115200|57600|38400|19200|9600|4800|2400|1800|1200|600|300|200|150|134|110|75|50
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baudRate")]
    pub baud_rate: Option<i64>,
    /// Required. Valid values are 8, 7, 6, 5.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataBits")]
    pub data_bits: Option<i64>,
    /// Required. Valid options are "none", "even", "odd". Defaults to "none".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity: Option<DeviceProtocolCommonComParity>,
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serialPort")]
    pub serial_port: Option<String>,
    /// Required. Bit that stops 1|2
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stopBits")]
    pub stop_bits: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceProtocolCommonComBaudRate {
    #[serde(rename = "115200")]
    r#_115200,
    #[serde(rename = "57600")]
    r#_57600,
    #[serde(rename = "38400")]
    r#_38400,
    #[serde(rename = "19200")]
    r#_19200,
    #[serde(rename = "9600")]
    r#_9600,
    #[serde(rename = "4800")]
    r#_4800,
    #[serde(rename = "2400")]
    r#_2400,
    #[serde(rename = "1800")]
    r#_1800,
    #[serde(rename = "1200")]
    r#_1200,
    #[serde(rename = "600")]
    r#_600,
    #[serde(rename = "300")]
    r#_300,
    #[serde(rename = "200")]
    r#_200,
    #[serde(rename = "150")]
    r#_150,
    #[serde(rename = "134")]
    r#_134,
    #[serde(rename = "110")]
    r#_110,
    #[serde(rename = "75")]
    r#_75,
    #[serde(rename = "50")]
    r#_50,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceProtocolCommonComDataBits {
    #[serde(rename = "8")]
    r#_8,
    #[serde(rename = "7")]
    r#_7,
    #[serde(rename = "6")]
    r#_6,
    #[serde(rename = "5")]
    r#_5,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceProtocolCommonComParity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "even")]
    Even,
    #[serde(rename = "odd")]
    Odd,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DeviceProtocolCommonComStopBits {
    #[serde(rename = "1")]
    r#_1,
    #[serde(rename = "2")]
    r#_2,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolCommonTcp {
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// Configuration for customized protocol
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolCustomizedProtocol {
    /// Any config data
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configData")]
    pub config_data: Option<BTreeMap<String, serde_json::Value>>,
    /// Unique protocol name Required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "protocolName")]
    pub protocol_name: Option<String>,
}

/// Protocol configuration for modbus
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolModbus {
    /// Required. 0-255
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "slaveID")]
    pub slave_id: Option<i64>,
}

/// Protocol configuration for opc-ua
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceProtocolOpcua {
    /// Certificate for access opc server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// Password for access opc server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// PrivateKey for access opc server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    /// Defaults to "none".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityMode")]
    pub security_mode: Option<String>,
    /// Defaults to "none".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityPolicy")]
    pub security_policy: Option<String>,
    /// Timeout seconds for the opc server connection.???
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Required: The URL for opc server endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Username for access opc server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userName")]
    pub user_name: Option<String>,
}

/// DeviceStatus reports the device state and the desired/reported values of twin attributes.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatus {
    /// A list of device twins containing desired/reported desired/reported values of twin properties. Optional: A passive device won't have twin properties and this list could be empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twins: Option<Vec<DeviceStatusTwins>>,
}

/// Twin provides a logical representation of control properties (writable properties in the device model). The properties can have a Desired state and a Reported state. The cloud configures the `Desired`state of a device property and this configuration update is pushed to the edge node. The mapper sends a command to the device to change this property value as per the desired state . It receives the `Reported` state of the property once the previous operation is complete and sends the reported state to the cloud. Offline device interaction in the edge is possible via twin properties for control/command operations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwins {
    /// Required: the desired property value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<DeviceStatusTwinsDesired>,
    /// Required: The property name for which the desired/reported values are specified. This property should be present in the device model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "propertyName")]
    pub property_name: Option<String>,
    /// Required: the reported property value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reported: Option<DeviceStatusTwinsReported>,
}

/// Required: the desired property value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwinsDesired {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The value for this property.
    pub value: String,
}

/// Required: the reported property value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DeviceStatusTwinsReported {
    /// Additional metadata like timestamp when the value was reported etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Required: The value for this property.
    pub value: String,
}

