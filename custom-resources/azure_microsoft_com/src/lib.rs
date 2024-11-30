/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated weekly, and released monthly.

# azure_microsoft_com

Custom resources in this crate belong to the `azure.microsoft.com` group. The following versions and custom resources are available:

## azure.microsoft.com/v1alpha1
- `APIMgmtAPI`
- `ApimService`
- `AppInsights`
- `AppInsightsApiKey`
- `AzureLoadBalancer`
- `AzureNetworkInterface`
- `AzurePublicIPAddress`
- `AzureSqlAction`
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSQLManagedUser`
- `AzureSqlServer`
- `AzureSQLUser`
- `AzureSQLVNetRule`
- `AzureVirtualMachineExtension`
- `AzureVirtualMachine`
- `AzureVMScaleSet`
- `BlobContainer`
- `ConsumerGroup`
- `CosmosDB`
- `EventhubNamespace`
- `Eventhub`
- `KeyVaultKey`
- `KeyVault`
- `MySQLAADUser`
- `MySQLDatabase`
- `MySQLFirewallRule`
- `MySQLServerAdministrator`
- `MySQLServer`
- `MySQLUser`
- `MySQLVNetRule`
- `PostgreSQLDatabase`
- `PostgreSQLFirewallRule`
- `PostgreSQLServer`
- `PostgreSQLUser`
- `PostgreSQLVNetRule`
- `RedisCacheAction`
- `RedisCacheFirewallRule`
- `ResourceGroup`
- `StorageAccount`
- `VirtualNetwork`
## azure.microsoft.com/v1alpha2
- `BlobContainer`
- `MySQLAADUser`
- `MySQLServer`
- `MySQLUser`
- `PostgreSQLServer`
## azure.microsoft.com/v1beta1
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSqlServer`
*/
pub mod v1alpha1;
pub mod v1alpha2;
pub mod v1beta1;
