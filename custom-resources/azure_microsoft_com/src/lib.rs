/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) for the `azure.microsoft.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

# Available Custom Resources

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
#[cfg(feature = "v1alpha1")]
pub mod v1alpha1;
#[cfg(feature = "v1alpha2")]
pub mod v1alpha2;
#[cfg(feature = "v1beta1")]
pub mod v1beta1;
