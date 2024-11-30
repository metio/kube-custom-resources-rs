<!--
SPDX-FileCopyrightText: The kube-custom-resources-rs Authors
SPDX-License-Identifier: 0BSD
 -->

# azure.microsoft.com

This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) of the `azure.microsoft.com` group. Each binding is generated with [kopium](https://github.com/kube-rs/kopium), updated and released weekly.

This crate is part of [kube-custom-resources-rs](https://github.com/metio/kube-custom-resources-rs).

## Available Custom Resources

### azure.microsoft.com/v1alpha1
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
### azure.microsoft.com/v1alpha2
- `BlobContainer`
- `MySQLAADUser`
- `MySQLServer`
- `MySQLUser`
- `PostgreSQLServer`
### azure.microsoft.com/v1beta1
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSqlServer`
