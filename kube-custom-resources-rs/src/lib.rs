/*!
This crate contains [kube-rs](https://kube.rs/) compatible bindings for Kubernetes [custom resources](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/). Each binding is generated with [kopium](https://github.com/kube-rs/kopium) and updated weekly.

# Available Features

Every group/version combination is its own feature in this crate. The available features are as follows:

## about_k8s_io_v1alpha1

apiVersion: `about.k8s.io/v1alpha1`

kinds:
- `ClusterProperty`

## acme_cert_manager_io_v1

apiVersion: `acme.cert-manager.io/v1`

kinds:
- `Challenge`
- `Order`

## addons_cluster_x_k8s_io_v1alpha4

apiVersion: `addons.cluster.x-k8s.io/v1alpha4`

kinds:
- `ClusterResourceSet`

## addons_cluster_x_k8s_io_v1beta1

apiVersion: `addons.cluster.x-k8s.io/v1beta1`

kinds:
- `ClusterResourceSet`

## agent_k8s_elastic_co_v1alpha1

apiVersion: `agent.k8s.elastic.co/v1alpha1`

kinds:
- `Agent`

## apicodegen_apimatic_io_v1beta1

apiVersion: `apicodegen.apimatic.io/v1beta1`

kinds:
- `APIMatic`

## apiextensions_crossplane_io_v1

apiVersion: `apiextensions.crossplane.io/v1`

kinds:
- `CompositeResourceDefinition`

## apigatewayv2_services_k8s_aws_v1alpha1

apiVersion: `apigatewayv2.services.k8s.aws/v1alpha1`

kinds:
- `API`
- `Authorizer`
- `Deployment`
- `Route`
- `Stage`
- `VPCLink`

## apm_k8s_elastic_co_v1

apiVersion: `apm.k8s.elastic.co/v1`

kinds:
- `ApmServer`

## apm_k8s_elastic_co_v1beta1

apiVersion: `apm.k8s.elastic.co/v1beta1`

kinds:
- `ApmServer`

## app_kiegroup_org_v1beta1

apiVersion: `app.kiegroup.org/v1beta1`

kinds:
- `KogitoBuild`
- `KogitoInfra`
- `KogitoRuntime`
- `KogitoSupportingService`

## app_lightbend_com_v1alpha1

apiVersion: `app.lightbend.com/v1alpha1`

kinds:
- `AkkaCluster`

## app_redislabs_com_v1

apiVersion: `app.redislabs.com/v1`

kinds:
- `RedisEnterpriseCluster`

## app_redislabs_com_v1alpha1

apiVersion: `app.redislabs.com/v1alpha1`

kinds:
- `RedisEnterpriseActiveActiveDatabase`
- `RedisEnterpriseCluster`
- `RedisEnterpriseDatabase`
- `RedisEnterpriseRemoteCluster`

## app_terraform_io_v1alpha2

apiVersion: `app.terraform.io/v1alpha2`

kinds:
- `AgentPool`
- `Module`
- `Workspace`

## applicationautoscaling_services_k8s_aws_v1alpha1

apiVersion: `applicationautoscaling.services.k8s.aws/v1alpha1`

kinds:
- `ScalableTarget`
- `ScalingPolicy`

## appprotect_f5_com_v1beta1

apiVersion: `appprotect.f5.com/v1beta1`

kinds:
- `APLogConf`
- `APUserSig`

## appprotectdos_f5_com_v1beta1

apiVersion: `appprotectdos.f5.com/v1beta1`

kinds:
- `APDosLogConf`
- `APDosPolicy`
- `DosProtectedResource`

## apps_3scale_net_v1alpha1

apiVersion: `apps.3scale.net/v1alpha1`

kinds:
- `APIcast`

## apps_clusternet_io_v1alpha1

apiVersion: `apps.clusternet.io/v1alpha1`

kinds:
- `Base`
- `Description`
- `FeedInventory`
- `Globalization`
- `HelmChart`
- `HelmRelease`
- `Localization`
- `Subscription`

## apps_gitlab_com_v1beta1

apiVersion: `apps.gitlab.com/v1beta1`

kinds:
- `GitLab`

## apps_kubeblocks_io_v1alpha1

apiVersion: `apps.kubeblocks.io/v1alpha1`

kinds:
- `BackupPolicyTemplate`
- `ClusterDefinition`
- `Cluster`
- `ClusterVersion`
- `ComponentClassDefinition`
- `ComponentResourceConstraint`
- `ConfigConstraint`
- `Configuration`
- `OpsRequest`
- `ServiceDescriptor`

## apps_kubedl_io_v1alpha1

apiVersion: `apps.kubedl.io/v1alpha1`

kinds:
- `Cron`

## apps_kubeedge_io_v1alpha1

apiVersion: `apps.kubeedge.io/v1alpha1`

kinds:
- `NodeGroup`

## apps_m88i_io_v1alpha1

apiVersion: `apps.m88i.io/v1alpha1`

kinds:
- `Nexus`

## aquasecurity_github_io_v1alpha1

apiVersion: `aquasecurity.github.io/v1alpha1`

kinds:
- `AquaStarboard`

## argoproj_io_v1alpha1

apiVersion: `argoproj.io/v1alpha1`

kinds:
- `Application`
- `AppProject`
- `ArgoCDExport`
- `ArgoCD`

## argoproj_io_v1beta1

apiVersion: `argoproj.io/v1beta1`

kinds:
- `ArgoCD`

## asdb_aerospike_com_v1

apiVersion: `asdb.aerospike.com/v1`

kinds:
- `AerospikeCluster`

## asdb_aerospike_com_v1beta1

apiVersion: `asdb.aerospike.com/v1beta1`

kinds:
- `AerospikeCluster`

## atlasmap_io_v1alpha1

apiVersion: `atlasmap.io/v1alpha1`

kinds:
- `AtlasMap`

## auth_ops42_org_v1alpha1

apiVersion: `auth.ops42.org/v1alpha1`

kinds:
- `AwsAuthSyncConfig`

## authzed_com_v1alpha1

apiVersion: `authzed.com/v1alpha1`

kinds:
- `SpiceDBCluster`

## autoscaling_k8s_io_v1

apiVersion: `autoscaling.k8s.io/v1`

kinds:
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`

## autoscaling_k8s_io_v1beta2

apiVersion: `autoscaling.k8s.io/v1beta2`

kinds:
- `VerticalPodAutoscalerCheckpoint`
- `VerticalPodAutoscaler`

## autoscaling_karmada_io_v1alpha1

apiVersion: `autoscaling.karmada.io/v1alpha1`

kinds:
- `CronFederatedHPA`
- `FederatedHPA`

## azure_microsoft_com_v1alpha1

apiVersion: `azure.microsoft.com/v1alpha1`

kinds:
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

## azure_microsoft_com_v1alpha2

apiVersion: `azure.microsoft.com/v1alpha2`

kinds:
- `BlobContainer`
- `MySQLAADUser`
- `MySQLServer`
- `MySQLUser`
- `PostgreSQLServer`

## azure_microsoft_com_v1beta1

apiVersion: `azure.microsoft.com/v1beta1`

kinds:
- `AzureSqlDatabase`
- `AzureSqlFailoverGroup`
- `AzureSqlFirewallRule`
- `AzureSqlServer`

## b3scale_infra_run_v1

apiVersion: `b3scale.infra.run/v1`

kinds:
- `BBBFrontend`

## batch_volcano_sh_v1alpha1

apiVersion: `batch.volcano.sh/v1alpha1`

kinds:
- `Job`

## beat_k8s_elastic_co_v1beta1

apiVersion: `beat.k8s.elastic.co/v1beta1`

kinds:
- `Beat`

## binding_operators_coreos_com_v1alpha1

apiVersion: `binding.operators.coreos.com/v1alpha1`

kinds:
- `BindableKinds`
- `ServiceBinding`

## bitnami_com_v1alpha1

apiVersion: `bitnami.com/v1alpha1`

kinds:
- `SealedSecret`

## boskos_k8s_io_v1

apiVersion: `boskos.k8s.io/v1`

kinds:
- `DRLCObject`
- `ResourceObject`

## bpfd_dev_v1alpha1

apiVersion: `bpfd.dev/v1alpha1`

kinds:
- `BpfProgram`
- `KprobeProgram`
- `TcProgram`
- `TracepointProgram`
- `UprobeProgram`
- `XdpProgram`

## bus_volcano_sh_v1alpha1

apiVersion: `bus.volcano.sh/v1alpha1`

kinds:
- `Command`

## cache_kubedl_io_v1alpha1

apiVersion: `cache.kubedl.io/v1alpha1`

kinds:
- `CacheBackend`

## caching_ibm_com_v1alpha1

apiVersion: `caching.ibm.com/v1alpha1`

kinds:
- `VarnishCluster`

## camel_apache_org_v1

apiVersion: `camel.apache.org/v1`

kinds:
- `Build`
- `CamelCatalog`
- `Kamelet`

## camel_apache_org_v1alpha1

apiVersion: `camel.apache.org/v1alpha1`

kinds:
- `Kamelet`

## capsule_clastix_io_v1alpha1

apiVersion: `capsule.clastix.io/v1alpha1`

kinds:
- `CapsuleConfiguration`
- `Tenant`

## capsule_clastix_io_v1beta1

apiVersion: `capsule.clastix.io/v1beta1`

kinds:
- `Tenant`

## capsule_clastix_io_v1beta2

apiVersion: `capsule.clastix.io/v1beta2`

kinds:
- `CapsuleConfiguration`
- `Tenant`

## ceph_rook_io_v1

apiVersion: `ceph.rook.io/v1`

kinds:
- `CephBlockPoolRadosNamespace`
- `CephBlockPool`
- `CephBucketNotification`
- `CephBucketTopic`
- `CephClient`
- `CephCOSIDriver`
- `CephFilesystemMirror`
- `CephFilesystem`
- `CephFilesystemSubVolumeGroup`
- `CephNFS`
- `CephObjectRealm`
- `CephObjectStore`
- `CephObjectStoreUser`
- `CephObjectZoneGroup`
- `CephObjectZone`
- `CephRBDMirror`

## cert_manager_io_v1

apiVersion: `cert-manager.io/v1`

kinds:
- `CertificateRequest`
- `Certificate`
- `ClusterIssuer`
- `Issuer`

## chaos_mesh_org_v1alpha1

apiVersion: `chaos-mesh.org/v1alpha1`

kinds:
- `AWSChaos`
- `AzureChaos`
- `BlockChaos`
- `DNSChaos`
- `GCPChaos`
- `HTTPChaos`
- `IOChaos`
- `JVMChaos`
- `KernelChaos`
- `NetworkChaos`
- `PhysicalMachineChaos`
- `PhysicalMachine`
- `PodChaos`
- `PodHttpChaos`
- `PodIOChaos`
- `PodNetworkChaos`
- `RemoteCluster`
- `Schedule`
- `StatusCheck`
- `StressChaos`
- `TimeChaos`
- `WorkflowNode`
- `Workflow`

## che_eclipse_org_v1alpha1

apiVersion: `che.eclipse.org/v1alpha1`

kinds:
- `KubernetesImagePuller`

## cilium_io_v2

apiVersion: `cilium.io/v2`

kinds:
- `CiliumClusterwideNetworkPolicy`
- `CiliumEgressGatewayPolicy`
- `CiliumEndpoint`
- `CiliumExternalWorkload`
- `CiliumNetworkPolicy`
- `CiliumNode`

## cilium_io_v2alpha1

apiVersion: `cilium.io/v2alpha1`

kinds:
- `CiliumBGPPeeringPolicy`
- `CiliumCIDRGroup`
- `CiliumEndpointSlice`
- `CiliumL2AnnouncementPolicy`
- `CiliumLoadBalancerIPPool`
- `CiliumNodeConfig`
- `CiliumPodIPPool`

## cloudformation_linki_space_v1alpha1

apiVersion: `cloudformation.linki.space/v1alpha1`

kinds:
- `Stack`

## cluster_clusterpedia_io_v1alpha2

apiVersion: `cluster.clusterpedia.io/v1alpha2`

kinds:
- `ClusterSyncResources`
- `PediaCluster`

## cluster_ipfs_io_v1alpha1

apiVersion: `cluster.ipfs.io/v1alpha1`

kinds:
- `CircuitRelay`
- `IpfsCluster`

## cluster_x_k8s_io_v1alpha4

apiVersion: `cluster.x-k8s.io/v1alpha4`

kinds:
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`

## cluster_x_k8s_io_v1beta1

apiVersion: `cluster.x-k8s.io/v1beta1`

kinds:
- `ClusterClass`
- `Cluster`
- `MachineDeployment`
- `MachineHealthCheck`
- `MachinePool`
- `Machine`
- `MachineSet`

## clusters_clusternet_io_v1beta1

apiVersion: `clusters.clusternet.io/v1beta1`

kinds:
- `ClusterRegistrationRequest`
- `ManagedCluster`

## config_gatekeeper_sh_v1alpha1

apiVersion: `config.gatekeeper.sh/v1alpha1`

kinds:
- `Config`

## config_grafana_com_v1

apiVersion: `config.grafana.com/v1`

kinds:
- `ProjectConfig`

## config_karmada_io_v1alpha1

apiVersion: `config.karmada.io/v1alpha1`

kinds:
- `ResourceInterpreterCustomization`
- `ResourceInterpreterWebhookConfiguration`

## config_koordinator_sh_v1alpha1

apiVersion: `config.koordinator.sh/v1alpha1`

kinds:
- `ClusterColocationProfile`

## core_linuxsuren_github_com_v1alpha1

apiVersion: `core.linuxsuren.github.com/v1alpha1`

kinds:
- `ATest`

## core_openfeature_dev_v1alpha1

apiVersion: `core.openfeature.dev/v1alpha1`

kinds:
- `FeatureFlagConfiguration`

## core_openfeature_dev_v1alpha2

apiVersion: `core.openfeature.dev/v1alpha2`

kinds:
- `FeatureFlagConfiguration`

## couchbase_com_v2

apiVersion: `couchbase.com/v2`

kinds:
- `CouchbaseAutoscaler`
- `CouchbaseBackupRestore`
- `CouchbaseBackup`
- `CouchbaseBucket`
- `CouchbaseCluster`
- `CouchbaseCollectionGroup`
- `CouchbaseCollection`
- `CouchbaseEphemeralBucket`
- `CouchbaseGroup`
- `CouchbaseMemcachedBucket`
- `CouchbaseMigrationReplication`
- `CouchbaseReplication`
- `CouchbaseRoleBinding`
- `CouchbaseScopeGroup`
- `CouchbaseScope`
- `CouchbaseUser`

## crd_projectcalico_org_v1

apiVersion: `crd.projectcalico.org/v1`

kinds:
- `BGPConfiguration`
- `BGPFilter`
- `BGPPeer`
- `BlockAffinity`
- `CalicoNodeStatus`
- `ClusterInformation`
- `GlobalNetworkSet`
- `HostEndpoint`
- `IPAMBlock`
- `IPAMConfig`
- `IPAMHandle`
- `IPPool`
- `IPReservation`
- `KubeControllersConfiguration`
- `NetworkSet`

## data_fluid_io_v1alpha1

apiVersion: `data.fluid.io/v1alpha1`

kinds:
- `AlluxioRuntime`
- `DataBackup`
- `DataLoad`
- `Dataset`
- `GooseFSRuntime`
- `JindoRuntime`
- `JuiceFSRuntime`
- `ThinRuntimeProfile`
- `ThinRuntime`

## databases_schemahero_io_v1alpha4

apiVersion: `databases.schemahero.io/v1alpha4`

kinds:
- `Database`

## dataprotection_kubeblocks_io_v1alpha1

apiVersion: `dataprotection.kubeblocks.io/v1alpha1`

kinds:
- `ActionSet`
- `BackupPolicy`
- `BackupRepo`
- `Backup`
- `BackupSchedule`
- `Restore`

## devices_kubeedge_io_v1alpha2

apiVersion: `devices.kubeedge.io/v1alpha2`

kinds:
- `DeviceModel`
- `Device`

## digitalis_io_v1

apiVersion: `digitalis.io/v1`

kinds:
- `ValsSecret`

## digitalis_io_v1beta1

apiVersion: `digitalis.io/v1beta1`

kinds:
- `DbSecret`

## druid_apache_org_v1alpha1

apiVersion: `druid.apache.org/v1alpha1`

kinds:
- `Druid`

## dynamodb_services_k8s_aws_v1alpha1

apiVersion: `dynamodb.services.k8s.aws/v1alpha1`

kinds:
- `Backup`
- `GlobalTable`
- `Table`

## ec2_services_k8s_aws_v1alpha1

apiVersion: `ec2.services.k8s.aws/v1alpha1`

kinds:
- `DHCPOptions`
- `ElasticIPAddress`
- `Instance`
- `InternetGateway`
- `NATGateway`
- `RouteTable`
- `SecurityGroup`
- `Subnet`
- `TransitGateway`
- `VPCEndpoint`
- `VPC`

## ecr_services_k8s_aws_v1alpha1

apiVersion: `ecr.services.k8s.aws/v1alpha1`

kinds:
- `PullThroughCacheRule`
- `Repository`

## eks_services_k8s_aws_v1alpha1

apiVersion: `eks.services.k8s.aws/v1alpha1`

kinds:
- `Addon`
- `Cluster`
- `FargateProfile`
- `Nodegroup`

## elasticache_services_k8s_aws_v1alpha1

apiVersion: `elasticache.services.k8s.aws/v1alpha1`

kinds:
- `CacheParameterGroup`
- `CacheSubnetGroup`
- `ReplicationGroup`
- `Snapshot`
- `UserGroup`
- `User`

## elasticsearch_k8s_elastic_co_v1

apiVersion: `elasticsearch.k8s.elastic.co/v1`

kinds:
- `Elasticsearch`

## elasticsearch_k8s_elastic_co_v1beta1

apiVersion: `elasticsearch.k8s.elastic.co/v1beta1`

kinds:
- `Elasticsearch`

## elbv2_k8s_aws_v1alpha1

apiVersion: `elbv2.k8s.aws/v1alpha1`

kinds:
- `TargetGroupBinding`

## elbv2_k8s_aws_v1beta1

apiVersion: `elbv2.k8s.aws/v1beta1`

kinds:
- `IngressClassParams`
- `TargetGroupBinding`

## emrcontainers_services_k8s_aws_v1alpha1

apiVersion: `emrcontainers.services.k8s.aws/v1alpha1`

kinds:
- `JobRun`
- `VirtualCluster`

## enterprisesearch_k8s_elastic_co_v1

apiVersion: `enterprisesearch.k8s.elastic.co/v1`

kinds:
- `EnterpriseSearch`

## enterprisesearch_k8s_elastic_co_v1beta1

apiVersion: `enterprisesearch.k8s.elastic.co/v1beta1`

kinds:
- `EnterpriseSearch`

## execution_furiko_io_v1alpha1

apiVersion: `execution.furiko.io/v1alpha1`

kinds:
- `JobConfig`
- `Job`

## executor_testkube_io_v1

apiVersion: `executor.testkube.io/v1`

kinds:
- `Executor`
- `Webhook`

## expansion_gatekeeper_sh_v1alpha1

apiVersion: `expansion.gatekeeper.sh/v1alpha1`

kinds:
- `ExpansionTemplate`

## expansion_gatekeeper_sh_v1beta1

apiVersion: `expansion.gatekeeper.sh/v1beta1`

kinds:
- `ExpansionTemplate`

## extensions_kubeblocks_io_v1alpha1

apiVersion: `extensions.kubeblocks.io/v1alpha1`

kinds:
- `Addon`

## external_secrets_io_v1alpha1

apiVersion: `external-secrets.io/v1alpha1`

kinds:
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`

## external_secrets_io_v1beta1

apiVersion: `external-secrets.io/v1beta1`

kinds:
- `ClusterExternalSecret`
- `ClusterSecretStore`
- `ExternalSecret`
- `SecretStore`

## externaldata_gatekeeper_sh_v1alpha1

apiVersion: `externaldata.gatekeeper.sh/v1alpha1`

kinds:
- `Provider`

## externaldata_gatekeeper_sh_v1beta1

apiVersion: `externaldata.gatekeeper.sh/v1beta1`

kinds:
- `Provider`

## externaldns_k8s_io_v1alpha1

apiVersion: `externaldns.k8s.io/v1alpha1`

kinds:
- `DNSEndpoint`

## externaldns_nginx_org_v1

apiVersion: `externaldns.nginx.org/v1`

kinds:
- `DNSEndpoint`

## flagger_app_v1beta1

apiVersion: `flagger.app/v1beta1`

kinds:
- `Canary`

## flink_apache_org_v1beta1

apiVersion: `flink.apache.org/v1beta1`

kinds:
- `FlinkDeployment`
- `FlinkSessionJob`

## flow_volcano_sh_v1alpha1

apiVersion: `flow.volcano.sh/v1alpha1`

kinds:
- `JobFlow`
- `JobTemplate`

## flows_netobserv_io_v1alpha1

apiVersion: `flows.netobserv.io/v1alpha1`

kinds:
- `FlowCollector`

## flows_netobserv_io_v1beta1

apiVersion: `flows.netobserv.io/v1beta1`

kinds:
- `FlowCollector`

## flows_netobserv_io_v1beta2

apiVersion: `flows.netobserv.io/v1beta2`

kinds:
- `FlowCollector`

## flux_framework_org_v1alpha1

apiVersion: `flux-framework.org/v1alpha1`

kinds:
- `MiniCluster`

## gateway_networking_k8s_io_v1

apiVersion: `gateway.networking.k8s.io/v1`

kinds:
- `GatewayClass`
- `Gateway`
- `HTTPRoute`

## gateway_networking_k8s_io_v1alpha2

apiVersion: `gateway.networking.k8s.io/v1alpha2`

kinds:
- `GRPCRoute`
- `ReferenceGrant`
- `TCPRoute`
- `TLSRoute`
- `UDPRoute`

## gateway_networking_k8s_io_v1beta1

apiVersion: `gateway.networking.k8s.io/v1beta1`

kinds:
- `GatewayClass`
- `Gateway`
- `HTTPRoute`
- `ReferenceGrant`

## gateway_nginx_org_v1alpha1

apiVersion: `gateway.nginx.org/v1alpha1`

kinds:
- `NginxGateway`

## getambassador_io_v3alpha1

apiVersion: `getambassador.io/v3alpha1`

kinds:
- `AuthService`
- `ConsulResolver`
- `DevPortal`
- `Host`
- `KubernetesEndpointResolver`
- `KubernetesServiceResolver`
- `Listener`
- `LogService`
- `Module`
- `RateLimitService`
- `TCPMapping`
- `TLSContext`
- `TracingService`

## gitops_hybrid_cloud_patterns_io_v1alpha1

apiVersion: `gitops.hybrid-cloud-patterns.io/v1alpha1`

kinds:
- `Pattern`

## grafana_integreatly_org_v1beta1

apiVersion: `grafana.integreatly.org/v1beta1`

kinds:
- `GrafanaDashboard`
- `GrafanaDatasource`
- `GrafanaFolder`

## hazelcast_com_v1alpha1

apiVersion: `hazelcast.com/v1alpha1`

kinds:
- `CronHotBackup`
- `Hazelcast`
- `HotBackup`
- `ManagementCenter`
- `Map`
- `WanReplication`

## helm_toolkit_fluxcd_io_v2beta1

apiVersion: `helm.toolkit.fluxcd.io/v2beta1`

kinds:
- `HelmRelease`

## hive_openshift_io_v1

apiVersion: `hive.openshift.io/v1`

kinds:
- `Checkpoint`
- `ClusterClaim`
- `ClusterDeploymentCustomization`
- `ClusterDeployment`
- `ClusterDeprovision`
- `ClusterImageSet`
- `ClusterPool`
- `ClusterProvision`
- `ClusterRelocate`
- `ClusterState`
- `DNSZone`
- `HiveConfig`
- `MachinePoolNameLease`
- `MachinePool`
- `SelectorSyncIdentityProvider`
- `SyncIdentityProvider`

## hiveinternal_openshift_io_v1alpha1

apiVersion: `hiveinternal.openshift.io/v1alpha1`

kinds:
- `ClusterSyncLease`
- `ClusterSync`
- `FakeClusterInstall`

## hnc_x_k8s_io_v1alpha2

apiVersion: `hnc.x-k8s.io/v1alpha2`

kinds:
- `HierarchicalResourceQuota`
- `HierarchyConfiguration`
- `HNCConfiguration`
- `SubnamespaceAnchor`

## hyperfoil_io_v1alpha1

apiVersion: `hyperfoil.io/v1alpha1`

kinds:
- `Horreum`

## hyperfoil_io_v1alpha2

apiVersion: `hyperfoil.io/v1alpha2`

kinds:
- `Hyperfoil`

## iam_services_k8s_aws_v1alpha1

apiVersion: `iam.services.k8s.aws/v1alpha1`

kinds:
- `Group`
- `Policy`
- `Role`

## ibmcloud_ibm_com_v1alpha1

apiVersion: `ibmcloud.ibm.com/v1alpha1`

kinds:
- `Composable`

## image_toolkit_fluxcd_io_v1beta1

apiVersion: `image.toolkit.fluxcd.io/v1beta1`

kinds:
- `ImageUpdateAutomation`
- `ImagePolicy`
- `ImageRepository`

## image_toolkit_fluxcd_io_v1beta2

apiVersion: `image.toolkit.fluxcd.io/v1beta2`

kinds:
- `ImagePolicy`
- `ImageRepository`

## imaging_ingestion_alvearie_org_v1alpha1

apiVersion: `imaging-ingestion.alvearie.org/v1alpha1`

kinds:
- `DicomEventBridge`
- `DicomEventDrivenIngestion`
- `DicomInstanceBinding`
- `DicomStudyBinding`
- `DicomwebIngestionService`
- `DimseIngestionService`
- `DimseProxy`

## inference_kubedl_io_v1alpha1

apiVersion: `inference.kubedl.io/v1alpha1`

kinds:
- `ElasticBatchJob`

## infinispan_org_v2alpha1

apiVersion: `infinispan.org/v2alpha1`

kinds:
- `Backup`
- `Batch`
- `Cache`
- `Restore`

## infrastructure_cluster_x_k8s_io_v1alpha1

apiVersion: `infrastructure.cluster.x-k8s.io/v1alpha1`

kinds:
- `KubevirtCluster`
- `KubevirtClusterTemplate`
- `KubevirtMachine`
- `KubevirtMachineTemplate`

## infrastructure_cluster_x_k8s_io_v1beta1

apiVersion: `infrastructure.cluster.x-k8s.io/v1beta1`

kinds:
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`
- `VSphereClusterIdentity`
- `VSphereCluster`
- `VSphereClusterTemplate`
- `VSphereDeploymentZone`
- `VSphereFailureDomain`
- `VSphereMachine`
- `VSphereMachineTemplate`
- `VSphereVM`

## infrastructure_cluster_x_k8s_io_v1beta2

apiVersion: `infrastructure.cluster.x-k8s.io/v1beta2`

kinds:
- `IBMPowerVSCluster`
- `IBMPowerVSClusterTemplate`
- `IBMPowerVSImage`
- `IBMPowerVSMachine`
- `IBMPowerVSMachineTemplate`
- `IBMVPCCluster`
- `IBMVPCMachine`
- `IBMVPCMachineTemplate`

## installation_mattermost_com_v1beta1

apiVersion: `installation.mattermost.com/v1beta1`

kinds:
- `Mattermost`

## integration_rock8s_com_v1beta1

apiVersion: `integration.rock8s.com/v1beta1`

kinds:
- `Plug`
- `Socket`

## iot_eclipse_org_v1alpha1

apiVersion: `iot.eclipse.org/v1alpha1`

kinds:
- `Ditto`
- `Hawkbit`

## ipam_cluster_x_k8s_io_v1alpha1

apiVersion: `ipam.cluster.x-k8s.io/v1alpha1`

kinds:
- `IPAddressClaim`
- `IPAddress`

## ipam_cluster_x_k8s_io_v1beta1

apiVersion: `ipam.cluster.x-k8s.io/v1beta1`

kinds:
- `IPAddressClaim`
- `IPAddress`

## jaegertracing_io_v1

apiVersion: `jaegertracing.io/v1`

kinds:
- `Jaeger`

## jobset_x_k8s_io_v1alpha2

apiVersion: `jobset.x-k8s.io/v1alpha2`

kinds:
- `JobSet`

## k8gb_absa_oss_v1beta1

apiVersion: `k8gb.absa.oss/v1beta1`

kinds:
- `Gslb`

## k8s_keycloak_org_v2alpha1

apiVersion: `k8s.keycloak.org/v2alpha1`

kinds:
- `KeycloakRealmImport`
- `Keycloak`

## k8s_nginx_org_v1

apiVersion: `k8s.nginx.org/v1`

kinds:
- `GlobalConfiguration`
- `Policy`
- `TransportServer`
- `VirtualServerRoute`
- `VirtualServer`

## k8s_nginx_org_v1alpha1

apiVersion: `k8s.nginx.org/v1alpha1`

kinds:
- `GlobalConfiguration`
- `Policy`
- `TransportServer`

## k8s_otterize_com_v1alpha2

apiVersion: `k8s.otterize.com/v1alpha2`

kinds:
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`

## k8s_otterize_com_v1alpha3

apiVersion: `k8s.otterize.com/v1alpha3`

kinds:
- `ClientIntents`
- `KafkaServerConfig`
- `ProtectedService`

## kafka_strimzi_io_v1alpha1

apiVersion: `kafka.strimzi.io/v1alpha1`

kinds:
- `KafkaTopic`
- `KafkaUser`

## kafka_strimzi_io_v1beta1

apiVersion: `kafka.strimzi.io/v1beta1`

kinds:
- `KafkaTopic`
- `KafkaUser`

## kafka_strimzi_io_v1beta2

apiVersion: `kafka.strimzi.io/v1beta2`

kinds:
- `KafkaBridge`
- `KafkaConnector`
- `KafkaConnect`
- `KafkaMirrorMaker`
- `KafkaRebalance`
- `Kafka`
- `KafkaTopic`
- `KafkaUser`

## keda_sh_v1alpha1

apiVersion: `keda.sh/v1alpha1`

kinds:
- `ClusterTriggerAuthentication`
- `ScaledJob`
- `ScaledObject`
- `TriggerAuthentication`

## keycloak_k8s_reddec_net_v1alpha1

apiVersion: `keycloak.k8s.reddec.net/v1alpha1`

kinds:
- `KeycloakClient`

## keycloak_org_v1alpha1

apiVersion: `keycloak.org/v1alpha1`

kinds:
- `KeycloakBackup`
- `KeycloakClient`
- `KeycloakRealm`
- `Keycloak`
- `KeycloakUser`

## kibana_k8s_elastic_co_v1

apiVersion: `kibana.k8s.elastic.co/v1`

kinds:
- `Kibana`

## kibana_k8s_elastic_co_v1beta1

apiVersion: `kibana.k8s.elastic.co/v1beta1`

kinds:
- `Kibana`

## kms_services_k8s_aws_v1alpha1

apiVersion: `kms.services.k8s.aws/v1alpha1`

kinds:
- `Alias`
- `Grant`
- `Key`

## kubean_io_v1alpha1

apiVersion: `kubean.io/v1alpha1`

kinds:
- `ClusterOperation`
- `Cluster`
- `Manifest`

## kubevious_io_v1alpha1

apiVersion: `kubevious.io/v1alpha1`

kinds:
- `WorkloadProfile`
- `Workload`

## kueue_x_k8s_io_v1beta1

apiVersion: `kueue.x-k8s.io/v1beta1`

kinds:
- `AdmissionCheck`
- `ClusterQueue`
- `LocalQueue`
- `ResourceFlavor`
- `Workload`

## kuma_io_v1alpha1

apiVersion: `kuma.io/v1alpha1`

kinds:
- `ContainerPatch`
- `MeshAccessLog`
- `MeshCircuitBreaker`
- `MeshFaultInjection`
- `MeshGatewayConfig`
- `MeshGatewayInstance`
- `MeshHealthCheck`
- `MeshHTTPRoute`
- `MeshLoadBalancingStrategy`
- `MeshProxyPatch`
- `MeshRateLimit`
- `MeshRetry`
- `MeshTCPRoute`
- `MeshTimeout`
- `MeshTrace`
- `MeshTrafficPermission`

## kustomize_toolkit_fluxcd_io_v1

apiVersion: `kustomize.toolkit.fluxcd.io/v1`

kinds:
- `Kustomization`

## kustomize_toolkit_fluxcd_io_v1beta1

apiVersion: `kustomize.toolkit.fluxcd.io/v1beta1`

kinds:
- `Kustomization`

## kustomize_toolkit_fluxcd_io_v1beta2

apiVersion: `kustomize.toolkit.fluxcd.io/v1beta2`

kinds:
- `Kustomization`

## kyverno_io_v1

apiVersion: `kyverno.io/v1`

kinds:
- `ClusterPolicy`
- `Policy`

## kyverno_io_v1alpha2

apiVersion: `kyverno.io/v1alpha2`

kinds:
- `AdmissionReport`
- `BackgroundScanReport`
- `ClusterAdmissionReport`
- `ClusterBackgroundScanReport`

## kyverno_io_v1beta1

apiVersion: `kyverno.io/v1beta1`

kinds:
- `UpdateRequest`

## kyverno_io_v2alpha1

apiVersion: `kyverno.io/v2alpha1`

kinds:
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `PolicyException`

## kyverno_io_v2beta1

apiVersion: `kyverno.io/v2beta1`

kinds:
- `CleanupPolicy`
- `ClusterCleanupPolicy`
- `ClusterPolicy`
- `Policy`
- `PolicyException`

## lambda_services_k8s_aws_v1alpha1

apiVersion: `lambda.services.k8s.aws/v1alpha1`

kinds:
- `CodeSigningConfig`
- `EventSourceMapping`
- `Function`
- `FunctionURLConfig`

## lerentis_uploadfilter24_eu_v1beta4

apiVersion: `lerentis.uploadfilter24.eu/v1beta4`

kinds:
- `BitwardenSecret`
- `BitwardenTemplate`
- `RegistryCredential`

## limitador_kuadrant_io_v1alpha1

apiVersion: `limitador.kuadrant.io/v1alpha1`

kinds:
- `Limitador`

## litmuschaos_io_v1alpha1

apiVersion: `litmuschaos.io/v1alpha1`

kinds:
- `ChaosEngine`
- `ChaosExperiment`

## logging_extensions_banzaicloud_io_v1alpha1

apiVersion: `logging-extensions.banzaicloud.io/v1alpha1`

kinds:
- `HostTailer`

## logging_banzaicloud_io_v1alpha1

apiVersion: `logging.banzaicloud.io/v1alpha1`

kinds:
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Logging`
- `Output`

## logging_banzaicloud_io_v1beta1

apiVersion: `logging.banzaicloud.io/v1beta1`

kinds:
- `ClusterFlow`
- `ClusterOutput`
- `Flow`
- `Output`
- `SyslogNGClusterFlow`
- `SyslogNGClusterOutput`
- `SyslogNGFlow`
- `SyslogNGOutput`

## loki_grafana_com_v1

apiVersion: `loki.grafana.com/v1`

kinds:
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`

## loki_grafana_com_v1beta1

apiVersion: `loki.grafana.com/v1beta1`

kinds:
- `AlertingRule`
- `LokiStack`
- `RecordingRule`
- `RulerConfig`

## longhorn_io_v1beta2

apiVersion: `longhorn.io/v1beta2`

kinds:
- `BackingImageDataSource`
- `BackingImageManager`
- `BackingImage`
- `Backup`
- `BackupTarget`
- `BackupVolume`
- `EngineImage`
- `Engine`
- `InstanceManager`
- `Node`
- `Orphan`
- `RecurringJob`
- `Replica`
- `ShareManager`
- `Snapshot`
- `SupportBundle`
- `SystemBackup`
- `SystemRestore`
- `VolumeAttachment`
- `Volume`

## maps_k8s_elastic_co_v1alpha1

apiVersion: `maps.k8s.elastic.co/v1alpha1`

kinds:
- `ElasticMapsServer`

## mariadb_mmontes_io_v1alpha1

apiVersion: `mariadb.mmontes.io/v1alpha1`

kinds:
- `Backup`
- `Connection`
- `Database`
- `Grant`
- `MariaDB`
- `Restore`
- `SqlJob`
- `User`

## mattermost_com_v1alpha1

apiVersion: `mattermost.com/v1alpha1`

kinds:
- `ClusterInstallation`
- `MattermostRestoreDB`

## metacontroller_k8s_io_v1alpha1

apiVersion: `metacontroller.k8s.io/v1alpha1`

kinds:
- `CompositeController`
- `ControllerRevision`
- `DecoratorController`

## metal3_io_v1alpha1

apiVersion: `metal3.io/v1alpha1`

kinds:
- `BMCEventSubscription`
- `FirmwareSchema`
- `HardwareData`
- `HostFirmwareSettings`
- `PreprovisioningImage`

## minio_min_io_v2

apiVersion: `minio.min.io/v2`

kinds:
- `Tenant`

## mirrors_kts_studio_v1alpha1

apiVersion: `mirrors.kts.studio/v1alpha1`

kinds:
- `SecretMirror`

## mirrors_kts_studio_v1alpha2

apiVersion: `mirrors.kts.studio/v1alpha2`

kinds:
- `SecretMirror`

## model_kubedl_io_v1alpha1

apiVersion: `model.kubedl.io/v1alpha1`

kinds:
- `Model`
- `ModelVersion`

## monitoring_coreos_com_v1

apiVersion: `monitoring.coreos.com/v1`

kinds:
- `Alertmanager`
- `PodMonitor`
- `Probe`
- `Prometheus`
- `PrometheusRule`
- `ServiceMonitor`
- `ThanosRuler`

## monitoring_coreos_com_v1alpha1

apiVersion: `monitoring.coreos.com/v1alpha1`

kinds:
- `AlertmanagerConfig`
- `PrometheusAgent`
- `ScrapeConfig`

## monitoring_coreos_com_v1beta1

apiVersion: `monitoring.coreos.com/v1beta1`

kinds:
- `AlertmanagerConfig`

## monocle_monocle_change_metrics_io_v1alpha1

apiVersion: `monocle.monocle.change-metrics.io/v1alpha1`

kinds:
- `Monocle`

## mq_services_k8s_aws_v1alpha1

apiVersion: `mq.services.k8s.aws/v1alpha1`

kinds:
- `Broker`

## multicluster_crd_antrea_io_v1alpha1

apiVersion: `multicluster.crd.antrea.io/v1alpha1`

kinds:
- `ClusterInfoImport`
- `ClusterSet`
- `Gateway`
- `LabelIdentity`
- `MemberClusterAnnounce`
- `MultiClusterConfig`
- `ResourceExport`
- `ResourceImport`

## multicluster_crd_antrea_io_v1alpha2

apiVersion: `multicluster.crd.antrea.io/v1alpha2`

kinds:
- `ClusterClaim`
- `ClusterSet`

## multicluster_x_k8s_io_v1alpha1

apiVersion: `multicluster.x-k8s.io/v1alpha1`

kinds:
- `ServiceExport`
- `ServiceImport`
- `AppliedWork`

## mutations_gatekeeper_sh_v1

apiVersion: `mutations.gatekeeper.sh/v1`

kinds:
- `Assign`
- `AssignMetadata`
- `ModifySet`

## mutations_gatekeeper_sh_v1alpha1

apiVersion: `mutations.gatekeeper.sh/v1alpha1`

kinds:
- `Assign`
- `AssignImage`
- `AssignMetadata`
- `ModifySet`

## mutations_gatekeeper_sh_v1beta1

apiVersion: `mutations.gatekeeper.sh/v1beta1`

kinds:
- `Assign`
- `AssignMetadata`
- `ModifySet`

## nativestor_alauda_io_v1

apiVersion: `nativestor.alauda.io/v1`

kinds:
- `RawDevice`

## networking_karmada_io_v1alpha1

apiVersion: `networking.karmada.io/v1alpha1`

kinds:
- `MultiClusterIngress`
- `MultiClusterService`

## nfd_k8s_sigs_io_v1alpha1

apiVersion: `nfd.k8s-sigs.io/v1alpha1`

kinds:
- `NodeFeatureRule`

## nfd_kubernetes_io_v1

apiVersion: `nfd.kubernetes.io/v1`

kinds:
- `NodeFeatureDiscovery`

## nfd_kubernetes_io_v1alpha1

apiVersion: `nfd.kubernetes.io/v1alpha1`

kinds:
- `NodeFeatureRule`

## nodeinfo_volcano_sh_v1alpha1

apiVersion: `nodeinfo.volcano.sh/v1alpha1`

kinds:
- `Numatopology`

## notebook_kubedl_io_v1alpha1

apiVersion: `notebook.kubedl.io/v1alpha1`

kinds:
- `Notebook`

## notification_toolkit_fluxcd_io_v1

apiVersion: `notification.toolkit.fluxcd.io/v1`

kinds:
- `Receiver`

## notification_toolkit_fluxcd_io_v1beta1

apiVersion: `notification.toolkit.fluxcd.io/v1beta1`

kinds:
- `Alert`
- `Provider`
- `Receiver`

## notification_toolkit_fluxcd_io_v1beta2

apiVersion: `notification.toolkit.fluxcd.io/v1beta2`

kinds:
- `Alert`
- `Provider`
- `Receiver`

## opensearchservice_services_k8s_aws_v1alpha1

apiVersion: `opensearchservice.services.k8s.aws/v1alpha1`

kinds:
- `Domain`

## opentelemetry_io_v1alpha1

apiVersion: `opentelemetry.io/v1alpha1`

kinds:
- `Instrumentation`
- `OpenTelemetryCollector`

## operations_kubeedge_io_v1alpha1

apiVersion: `operations.kubeedge.io/v1alpha1`

kinds:
- `NodeUpgradeJob`

## operator_aquasec_com_v1alpha1

apiVersion: `operator.aquasec.com/v1alpha1`

kinds:
- `AquaCsp`
- `AquaDatabase`
- `AquaEnforcer`
- `AquaGateway`
- `AquaKubeEnforcer`
- `AquaScanner`
- `AquaServer`

## operator_authorino_kuadrant_io_v1beta1

apiVersion: `operator.authorino.kuadrant.io/v1beta1`

kinds:
- `Authorino`

## operator_cluster_x_k8s_io_v1alpha1

apiVersion: `operator.cluster.x-k8s.io/v1alpha1`

kinds:
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`

## operator_cluster_x_k8s_io_v1alpha2

apiVersion: `operator.cluster.x-k8s.io/v1alpha2`

kinds:
- `AddonProvider`
- `BootstrapProvider`
- `ControlPlaneProvider`
- `CoreProvider`
- `InfrastructureProvider`

## operator_cryostat_io_v1beta1

apiVersion: `operator.cryostat.io/v1beta1`

kinds:
- `Cryostat`

## operator_open_cluster_management_io_v1

apiVersion: `operator.open-cluster-management.io/v1`

kinds:
- `ClusterManager`
- `Klusterlet`

## operator_shipwright_io_v1alpha1

apiVersion: `operator.shipwright.io/v1alpha1`

kinds:
- `ShipwrightBuild`

## operator_tigera_io_v1

apiVersion: `operator.tigera.io/v1`

kinds:
- `APIServer`
- `Installation`
- `TigeraStatus`

## operator_victoriametrics_com_v1beta1

apiVersion: `operator.victoriametrics.com/v1beta1`

kinds:
- `VMRule`
- `VMUser`

## org_eclipse_che_v1

apiVersion: `org.eclipse.che/v1`

kinds:
- `CheCluster`

## org_eclipse_che_v2

apiVersion: `org.eclipse.che/v2`

kinds:
- `CheCluster`

## pkg_crossplane_io_v1

apiVersion: `pkg.crossplane.io/v1`

kinds:
- `ConfigurationRevision`
- `Configuration`
- `ProviderRevision`
- `Provider`

## pkg_crossplane_io_v1alpha1

apiVersion: `pkg.crossplane.io/v1alpha1`

kinds:
- `ControllerConfig`

## pkg_crossplane_io_v1beta1

apiVersion: `pkg.crossplane.io/v1beta1`

kinds:
- `Lock`

## policy_clusterpedia_io_v1alpha1

apiVersion: `policy.clusterpedia.io/v1alpha1`

kinds:
- `ClusterImportPolicy`
- `PediaClusterLifecycle`

## policy_karmada_io_v1alpha1

apiVersion: `policy.karmada.io/v1alpha1`

kinds:
- `ClusterOverridePolicy`
- `ClusterPropagationPolicy`
- `FederatedResourceQuota`
- `OverridePolicy`
- `PropagationPolicy`

## postgres_operator_crunchydata_com_v1beta1

apiVersion: `postgres-operator.crunchydata.com/v1beta1`

kinds:
- `PGAdmin`
- `PGUpgrade`
- `PostgresCluster`

## postgresql_cnpg_io_v1

apiVersion: `postgresql.cnpg.io/v1`

kinds:
- `Backup`
- `Pooler`
- `ScheduledBackup`

## prometheusservice_services_k8s_aws_v1alpha1

apiVersion: `prometheusservice.services.k8s.aws/v1alpha1`

kinds:
- `AlertManagerDefinition`
- `RuleGroupsNamespace`
- `Workspace`

## quay_redhat_com_v1

apiVersion: `quay.redhat.com/v1`

kinds:
- `QuayRegistry`

## ray_io_v1

apiVersion: `ray.io/v1`

kinds:
- `RayCluster`
- `RayJob`
- `RayService`

## ray_io_v1alpha1

apiVersion: `ray.io/v1alpha1`

kinds:
- `RayCluster`
- `RayJob`
- `RayService`

## rds_services_k8s_aws_v1alpha1

apiVersion: `rds.services.k8s.aws/v1alpha1`

kinds:
- `DBClusterParameterGroup`
- `DBCluster`
- `DBInstance`
- `DBParameterGroup`
- `DBProxy`
- `DBSubnetGroup`
- `GlobalCluster`

## redhatcop_redhat_io_v1alpha1

apiVersion: `redhatcop.redhat.io/v1alpha1`

kinds:
- `GroupConfig`
- `NamespaceConfig`
- `UserConfig`

## registry_apicur_io_v1

apiVersion: `registry.apicur.io/v1`

kinds:
- `ApicurioRegistry`

## registry_devfile_io_v1alpha1

apiVersion: `registry.devfile.io/v1alpha1`

kinds:
- `ClusterDevfileRegistriesList`
- `DevfileRegistry`
- `DevfileRegistriesList`

## reliablesyncs_kubeedge_io_v1alpha1

apiVersion: `reliablesyncs.kubeedge.io/v1alpha1`

kinds:
- `ClusterObjectSync`
- `ObjectSync`

## repo_manager_pulpproject_org_v1beta2

apiVersion: `repo-manager.pulpproject.org/v1beta2`

kinds:
- `PulpBackup`
- `PulpRestore`

## resources_teleport_dev_v1

apiVersion: `resources.teleport.dev/v1`

kinds:
- `TeleportLoginRule`
- `TeleportOktaImportRule`

## resources_teleport_dev_v2

apiVersion: `resources.teleport.dev/v2`

kinds:
- `TeleportSAMLConnector`
- `TeleportUser`

## resources_teleport_dev_v3

apiVersion: `resources.teleport.dev/v3`

kinds:
- `TeleportGithubConnector`
- `TeleportOIDCConnector`

## rocketmq_apache_org_v1alpha1

apiVersion: `rocketmq.apache.org/v1alpha1`

kinds:
- `Broker`
- `Console`
- `NameService`
- `TopicTransfer`

## rules_kubeedge_io_v1

apiVersion: `rules.kubeedge.io/v1`

kinds:
- `RuleEndpoint`
- `Rule`

## runtime_cluster_x_k8s_io_v1alpha1

apiVersion: `runtime.cluster.x-k8s.io/v1alpha1`

kinds:
- `ExtensionConfig`

## s3_services_k8s_aws_v1alpha1

apiVersion: `s3.services.k8s.aws/v1alpha1`

kinds:
- `Bucket`

## sagemaker_services_k8s_aws_v1alpha1

apiVersion: `sagemaker.services.k8s.aws/v1alpha1`

kinds:
- `App`
- `DataQualityJobDefinition`
- `Domain`
- `EndpointConfig`
- `Endpoint`
- `FeatureGroup`
- `HyperParameterTuningJob`
- `ModelBiasJobDefinition`
- `ModelExplainabilityJobDefinition`
- `ModelPackageGroup`
- `ModelPackage`
- `ModelQualityJobDefinition`
- `Model`
- `MonitoringSchedule`
- `NotebookInstanceLifecycleConfig`
- `NotebookInstance`
- `ProcessingJob`
- `TrainingJob`
- `TransformJob`
- `UserProfile`

## scheduling_koordinator_sh_v1alpha1

apiVersion: `scheduling.koordinator.sh/v1alpha1`

kinds:
- `Device`
- `PodMigrationJob`
- `Reservation`

## scheduling_sigs_k8s_io_v1alpha1

apiVersion: `scheduling.sigs.k8s.io/v1alpha1`

kinds:
- `ElasticQuota`
- `PodGroup`

## scheduling_volcano_sh_v1beta1

apiVersion: `scheduling.volcano.sh/v1beta1`

kinds:
- `PodGroup`
- `Queue`

## schemas_schemahero_io_v1alpha4

apiVersion: `schemas.schemahero.io/v1alpha4`

kinds:
- `DataType`
- `Migration`
- `Table`

## scylla_scylladb_com_v1

apiVersion: `scylla.scylladb.com/v1`

kinds:
- `ScyllaCluster`

## scylla_scylladb_com_v1alpha1

apiVersion: `scylla.scylladb.com/v1alpha1`

kinds:
- `NodeConfig`
- `ScyllaOperatorConfig`

## secretgenerator_mittwald_de_v1alpha1

apiVersion: `secretgenerator.mittwald.de/v1alpha1`

kinds:
- `BasicAuth`
- `SSHKeyPair`
- `StringSecret`

## secrets_crossplane_io_v1alpha1

apiVersion: `secrets.crossplane.io/v1alpha1`

kinds:
- `StoreConfig`

## secscan_quay_redhat_com_v1alpha1

apiVersion: `secscan.quay.redhat.com/v1alpha1`

kinds:
- `ImageManifestVuln`

## security_profiles_operator_x_k8s_io_v1alpha1

apiVersion: `security-profiles-operator.x-k8s.io/v1alpha1`

kinds:
- `AppArmorProfile`
- `ProfileBinding`
- `ProfileRecording`
- `SecurityProfileNodeStatus`
- `SecurityProfilesOperatorDaemon`

## security_profiles_operator_x_k8s_io_v1alpha2

apiVersion: `security-profiles-operator.x-k8s.io/v1alpha2`

kinds:
- `RawSelinuxProfile`

## security_profiles_operator_x_k8s_io_v1beta1

apiVersion: `security-profiles-operator.x-k8s.io/v1beta1`

kinds:
- `SeccompProfile`

## servicebinding_io_v1alpha3

apiVersion: `servicebinding.io/v1alpha3`

kinds:
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`

## servicebinding_io_v1beta1

apiVersion: `servicebinding.io/v1beta1`

kinds:
- `ClusterWorkloadResourceMapping`
- `ServiceBinding`

## services_k8s_aws_v1alpha1

apiVersion: `services.k8s.aws/v1alpha1`

kinds:
- `AdoptedResource`
- `FieldExport`

## serving_kubedl_io_v1alpha1

apiVersion: `serving.kubedl.io/v1alpha1`

kinds:
- `Inference`

## sfn_services_k8s_aws_v1alpha1

apiVersion: `sfn.services.k8s.aws/v1alpha1`

kinds:
- `Activity`
- `StateMachine`

## site_superedge_io_v1alpha1

apiVersion: `site.superedge.io/v1alpha1`

kinds:
- `NodeGroup`
- `NodeUnit`

## slo_koordinator_sh_v1alpha1

apiVersion: `slo.koordinator.sh/v1alpha1`

kinds:
- `NodeMetric`
- `NodeSLO`

## sonataflow_org_v1alpha08

apiVersion: `sonataflow.org/v1alpha08`

kinds:
- `SonataFlowBuild`
- `SonataFlowPlatform`

## source_toolkit_fluxcd_io_v1beta1

apiVersion: `source.toolkit.fluxcd.io/v1beta1`

kinds:
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`

## source_toolkit_fluxcd_io_v1beta2

apiVersion: `source.toolkit.fluxcd.io/v1beta2`

kinds:
- `Bucket`
- `GitRepository`
- `HelmChart`
- `HelmRepository`
- `OCIRepository`

## sparkoperator_k8s_io_v1beta2

apiVersion: `sparkoperator.k8s.io/v1beta2`

kinds:
- `ScheduledSparkApplication`
- `SparkApplication`

## status_gatekeeper_sh_v1beta1

apiVersion: `status.gatekeeper.sh/v1beta1`

kinds:
- `ConstraintPodStatus`
- `ConstraintTemplatePodStatus`
- `ExpansionTemplatePodStatus`
- `MutatorPodStatus`

## storage_kubeblocks_io_v1alpha1

apiVersion: `storage.kubeblocks.io/v1alpha1`

kinds:
- `StorageProvider`

## sts_min_io_v1alpha1

apiVersion: `sts.min.io/v1alpha1`

kinds:
- `PolicyBinding`

## stunner_l7mp_io_v1alpha1

apiVersion: `stunner.l7mp.io/v1alpha1`

kinds:
- `Dataplane`
- `GatewayConfig`
- `StaticService`

## submariner_io_v1alpha1

apiVersion: `submariner.io/v1alpha1`

kinds:
- `Broker`
- `ServiceDiscovery`
- `Submariner`

## templates_gatekeeper_sh_v1

apiVersion: `templates.gatekeeper.sh/v1`

kinds:
- `ConstraintTemplate`

## templates_gatekeeper_sh_v1alpha1

apiVersion: `templates.gatekeeper.sh/v1alpha1`

kinds:
- `ConstraintTemplate`

## templates_gatekeeper_sh_v1beta1

apiVersion: `templates.gatekeeper.sh/v1beta1`

kinds:
- `ConstraintTemplate`

## tests_testkube_io_v1

apiVersion: `tests.testkube.io/v1`

kinds:
- `Script`
- `TestExecution`
- `Test`
- `TestSource`
- `TestSuiteExecution`
- `TestSuite`
- `TestTrigger`

## tests_testkube_io_v2

apiVersion: `tests.testkube.io/v2`

kinds:
- `Script`
- `Test`
- `TestSuite`

## tests_testkube_io_v3

apiVersion: `tests.testkube.io/v3`

kinds:
- `Test`
- `TestSuite`

## topology_node_k8s_io_v1alpha1

apiVersion: `topology.node.k8s.io/v1alpha1`

kinds:
- `NodeResourceTopology`

## topolvm_cybozu_com_v1

apiVersion: `topolvm.cybozu.com/v1`

kinds:
- `LogicalVolume`

## topolvm_cybozu_com_v2

apiVersion: `topolvm.cybozu.com/v2`

kinds:
- `TopolvmCluster`

## traefik_io_v1alpha1

apiVersion: `traefik.io/v1alpha1`

kinds:
- `IngressRoute`
- `IngressRouteTCP`
- `IngressRouteUDP`
- `MiddlewareTCP`
- `ServersTransport`
- `ServersTransportTCP`
- `TLSOption`
- `TLSStore`
- `TraefikService`

## training_kubedl_io_v1alpha1

apiVersion: `training.kubedl.io/v1alpha1`

kinds:
- `ElasticDLJob`
- `MarsJob`
- `MPIJob`
- `PyTorchJob`
- `TFJob`
- `XDLJob`
- `XGBoostJob`

## virt_virtink_smartx_com_v1alpha1

apiVersion: `virt.virtink.smartx.com/v1alpha1`

kinds:
- `VirtualMachineMigration`
- `VirtualMachine`

## wgpolicyk8s_io_v1alpha1

apiVersion: `wgpolicyk8s.io/v1alpha1`

kinds:
- `ClusterPolicyReport`
- `PolicyReport`

## wgpolicyk8s_io_v1alpha2

apiVersion: `wgpolicyk8s.io/v1alpha2`

kinds:
- `ClusterPolicyReport`
- `PolicyReport`

## wgpolicyk8s_io_v1beta1

apiVersion: `wgpolicyk8s.io/v1beta1`

kinds:
- `ClusterPolicyReport`
- `PolicyReport`

## wildfly_org_v1alpha1

apiVersion: `wildfly.org/v1alpha1`

kinds:
- `WildFlyServer`

## work_karmada_io_v1alpha1

apiVersion: `work.karmada.io/v1alpha1`

kinds:
- `ClusterResourceBinding`
- `ResourceBinding`

## work_karmada_io_v1alpha2

apiVersion: `work.karmada.io/v1alpha2`

kinds:
- `ClusterResourceBinding`
- `ResourceBinding`

## workloads_kubeblocks_io_v1alpha1

apiVersion: `workloads.kubeblocks.io/v1alpha1`

kinds:
- `ReplicatedStateMachine`
 */

#[cfg(feature = "about_k8s_io_v1alpha1")]
pub mod about_k8s_io_v1alpha1;
#[cfg(feature = "acme_cert_manager_io_v1")]
pub mod acme_cert_manager_io_v1;
#[cfg(feature = "addons_cluster_x_k8s_io_v1alpha4")]
pub mod addons_cluster_x_k8s_io_v1alpha4;
#[cfg(feature = "addons_cluster_x_k8s_io_v1beta1")]
pub mod addons_cluster_x_k8s_io_v1beta1;
#[cfg(feature = "agent_k8s_elastic_co_v1alpha1")]
pub mod agent_k8s_elastic_co_v1alpha1;
#[cfg(feature = "apicodegen_apimatic_io_v1beta1")]
pub mod apicodegen_apimatic_io_v1beta1;
#[cfg(feature = "apiextensions_crossplane_io_v1")]
pub mod apiextensions_crossplane_io_v1;
#[cfg(feature = "apigatewayv2_services_k8s_aws_v1alpha1")]
pub mod apigatewayv2_services_k8s_aws_v1alpha1;
#[cfg(feature = "apm_k8s_elastic_co_v1")]
pub mod apm_k8s_elastic_co_v1;
#[cfg(feature = "apm_k8s_elastic_co_v1beta1")]
pub mod apm_k8s_elastic_co_v1beta1;
#[cfg(feature = "app_kiegroup_org_v1beta1")]
pub mod app_kiegroup_org_v1beta1;
#[cfg(feature = "app_lightbend_com_v1alpha1")]
pub mod app_lightbend_com_v1alpha1;
#[cfg(feature = "app_redislabs_com_v1")]
pub mod app_redislabs_com_v1;
#[cfg(feature = "app_redislabs_com_v1alpha1")]
pub mod app_redislabs_com_v1alpha1;
#[cfg(feature = "app_terraform_io_v1alpha2")]
pub mod app_terraform_io_v1alpha2;
#[cfg(feature = "applicationautoscaling_services_k8s_aws_v1alpha1")]
pub mod applicationautoscaling_services_k8s_aws_v1alpha1;
#[cfg(feature = "appprotect_f5_com_v1beta1")]
pub mod appprotect_f5_com_v1beta1;
#[cfg(feature = "appprotectdos_f5_com_v1beta1")]
pub mod appprotectdos_f5_com_v1beta1;
#[cfg(feature = "apps_3scale_net_v1alpha1")]
pub mod apps_3scale_net_v1alpha1;
#[cfg(feature = "apps_clusternet_io_v1alpha1")]
pub mod apps_clusternet_io_v1alpha1;
#[cfg(feature = "apps_gitlab_com_v1beta1")]
pub mod apps_gitlab_com_v1beta1;
#[cfg(feature = "apps_kubeblocks_io_v1alpha1")]
pub mod apps_kubeblocks_io_v1alpha1;
#[cfg(feature = "apps_kubedl_io_v1alpha1")]
pub mod apps_kubedl_io_v1alpha1;
#[cfg(feature = "apps_kubeedge_io_v1alpha1")]
pub mod apps_kubeedge_io_v1alpha1;
#[cfg(feature = "apps_m88i_io_v1alpha1")]
pub mod apps_m88i_io_v1alpha1;
#[cfg(feature = "aquasecurity_github_io_v1alpha1")]
pub mod aquasecurity_github_io_v1alpha1;
#[cfg(feature = "argoproj_io_v1alpha1")]
pub mod argoproj_io_v1alpha1;
#[cfg(feature = "argoproj_io_v1beta1")]
pub mod argoproj_io_v1beta1;
#[cfg(feature = "asdb_aerospike_com_v1")]
pub mod asdb_aerospike_com_v1;
#[cfg(feature = "asdb_aerospike_com_v1beta1")]
pub mod asdb_aerospike_com_v1beta1;
#[cfg(feature = "atlasmap_io_v1alpha1")]
pub mod atlasmap_io_v1alpha1;
#[cfg(feature = "auth_ops42_org_v1alpha1")]
pub mod auth_ops42_org_v1alpha1;
#[cfg(feature = "authzed_com_v1alpha1")]
pub mod authzed_com_v1alpha1;
#[cfg(feature = "autoscaling_k8s_io_v1")]
pub mod autoscaling_k8s_io_v1;
#[cfg(feature = "autoscaling_k8s_io_v1beta2")]
pub mod autoscaling_k8s_io_v1beta2;
#[cfg(feature = "autoscaling_karmada_io_v1alpha1")]
pub mod autoscaling_karmada_io_v1alpha1;
#[cfg(feature = "azure_microsoft_com_v1alpha1")]
pub mod azure_microsoft_com_v1alpha1;
#[cfg(feature = "azure_microsoft_com_v1alpha2")]
pub mod azure_microsoft_com_v1alpha2;
#[cfg(feature = "azure_microsoft_com_v1beta1")]
pub mod azure_microsoft_com_v1beta1;
#[cfg(feature = "b3scale_infra_run_v1")]
pub mod b3scale_infra_run_v1;
#[cfg(feature = "batch_volcano_sh_v1alpha1")]
pub mod batch_volcano_sh_v1alpha1;
#[cfg(feature = "beat_k8s_elastic_co_v1beta1")]
pub mod beat_k8s_elastic_co_v1beta1;
#[cfg(feature = "binding_operators_coreos_com_v1alpha1")]
pub mod binding_operators_coreos_com_v1alpha1;
#[cfg(feature = "bitnami_com_v1alpha1")]
pub mod bitnami_com_v1alpha1;
#[cfg(feature = "boskos_k8s_io_v1")]
pub mod boskos_k8s_io_v1;
#[cfg(feature = "bpfd_dev_v1alpha1")]
pub mod bpfd_dev_v1alpha1;
#[cfg(feature = "bus_volcano_sh_v1alpha1")]
pub mod bus_volcano_sh_v1alpha1;
#[cfg(feature = "cache_kubedl_io_v1alpha1")]
pub mod cache_kubedl_io_v1alpha1;
#[cfg(feature = "caching_ibm_com_v1alpha1")]
pub mod caching_ibm_com_v1alpha1;
#[cfg(feature = "camel_apache_org_v1")]
pub mod camel_apache_org_v1;
#[cfg(feature = "camel_apache_org_v1alpha1")]
pub mod camel_apache_org_v1alpha1;
#[cfg(feature = "capsule_clastix_io_v1alpha1")]
pub mod capsule_clastix_io_v1alpha1;
#[cfg(feature = "capsule_clastix_io_v1beta1")]
pub mod capsule_clastix_io_v1beta1;
#[cfg(feature = "capsule_clastix_io_v1beta2")]
pub mod capsule_clastix_io_v1beta2;
#[cfg(feature = "ceph_rook_io_v1")]
pub mod ceph_rook_io_v1;
#[cfg(feature = "cert_manager_io_v1")]
pub mod cert_manager_io_v1;
#[cfg(feature = "chaos_mesh_org_v1alpha1")]
pub mod chaos_mesh_org_v1alpha1;
#[cfg(feature = "che_eclipse_org_v1alpha1")]
pub mod che_eclipse_org_v1alpha1;
#[cfg(feature = "cilium_io_v2")]
pub mod cilium_io_v2;
#[cfg(feature = "cilium_io_v2alpha1")]
pub mod cilium_io_v2alpha1;
#[cfg(feature = "cloudformation_linki_space_v1alpha1")]
pub mod cloudformation_linki_space_v1alpha1;
#[cfg(feature = "cluster_clusterpedia_io_v1alpha2")]
pub mod cluster_clusterpedia_io_v1alpha2;
#[cfg(feature = "cluster_ipfs_io_v1alpha1")]
pub mod cluster_ipfs_io_v1alpha1;
#[cfg(feature = "cluster_x_k8s_io_v1alpha4")]
pub mod cluster_x_k8s_io_v1alpha4;
#[cfg(feature = "cluster_x_k8s_io_v1beta1")]
pub mod cluster_x_k8s_io_v1beta1;
#[cfg(feature = "clusters_clusternet_io_v1beta1")]
pub mod clusters_clusternet_io_v1beta1;
#[cfg(feature = "config_gatekeeper_sh_v1alpha1")]
pub mod config_gatekeeper_sh_v1alpha1;
#[cfg(feature = "config_grafana_com_v1")]
pub mod config_grafana_com_v1;
#[cfg(feature = "config_karmada_io_v1alpha1")]
pub mod config_karmada_io_v1alpha1;
#[cfg(feature = "config_koordinator_sh_v1alpha1")]
pub mod config_koordinator_sh_v1alpha1;
#[cfg(feature = "core_linuxsuren_github_com_v1alpha1")]
pub mod core_linuxsuren_github_com_v1alpha1;
#[cfg(feature = "core_openfeature_dev_v1alpha1")]
pub mod core_openfeature_dev_v1alpha1;
#[cfg(feature = "core_openfeature_dev_v1alpha2")]
pub mod core_openfeature_dev_v1alpha2;
#[cfg(feature = "couchbase_com_v2")]
pub mod couchbase_com_v2;
#[cfg(feature = "crd_projectcalico_org_v1")]
pub mod crd_projectcalico_org_v1;
#[cfg(feature = "data_fluid_io_v1alpha1")]
pub mod data_fluid_io_v1alpha1;
#[cfg(feature = "databases_schemahero_io_v1alpha4")]
pub mod databases_schemahero_io_v1alpha4;
#[cfg(feature = "dataprotection_kubeblocks_io_v1alpha1")]
pub mod dataprotection_kubeblocks_io_v1alpha1;
#[cfg(feature = "devices_kubeedge_io_v1alpha2")]
pub mod devices_kubeedge_io_v1alpha2;
#[cfg(feature = "digitalis_io_v1")]
pub mod digitalis_io_v1;
#[cfg(feature = "digitalis_io_v1beta1")]
pub mod digitalis_io_v1beta1;
#[cfg(feature = "druid_apache_org_v1alpha1")]
pub mod druid_apache_org_v1alpha1;
#[cfg(feature = "dynamodb_services_k8s_aws_v1alpha1")]
pub mod dynamodb_services_k8s_aws_v1alpha1;
#[cfg(feature = "ec2_services_k8s_aws_v1alpha1")]
pub mod ec2_services_k8s_aws_v1alpha1;
#[cfg(feature = "ecr_services_k8s_aws_v1alpha1")]
pub mod ecr_services_k8s_aws_v1alpha1;
#[cfg(feature = "eks_services_k8s_aws_v1alpha1")]
pub mod eks_services_k8s_aws_v1alpha1;
#[cfg(feature = "elasticache_services_k8s_aws_v1alpha1")]
pub mod elasticache_services_k8s_aws_v1alpha1;
#[cfg(feature = "elasticsearch_k8s_elastic_co_v1")]
pub mod elasticsearch_k8s_elastic_co_v1;
#[cfg(feature = "elasticsearch_k8s_elastic_co_v1beta1")]
pub mod elasticsearch_k8s_elastic_co_v1beta1;
#[cfg(feature = "elbv2_k8s_aws_v1alpha1")]
pub mod elbv2_k8s_aws_v1alpha1;
#[cfg(feature = "elbv2_k8s_aws_v1beta1")]
pub mod elbv2_k8s_aws_v1beta1;
#[cfg(feature = "emrcontainers_services_k8s_aws_v1alpha1")]
pub mod emrcontainers_services_k8s_aws_v1alpha1;
#[cfg(feature = "enterprisesearch_k8s_elastic_co_v1")]
pub mod enterprisesearch_k8s_elastic_co_v1;
#[cfg(feature = "enterprisesearch_k8s_elastic_co_v1beta1")]
pub mod enterprisesearch_k8s_elastic_co_v1beta1;
#[cfg(feature = "execution_furiko_io_v1alpha1")]
pub mod execution_furiko_io_v1alpha1;
#[cfg(feature = "executor_testkube_io_v1")]
pub mod executor_testkube_io_v1;
#[cfg(feature = "expansion_gatekeeper_sh_v1alpha1")]
pub mod expansion_gatekeeper_sh_v1alpha1;
#[cfg(feature = "expansion_gatekeeper_sh_v1beta1")]
pub mod expansion_gatekeeper_sh_v1beta1;
#[cfg(feature = "extensions_kubeblocks_io_v1alpha1")]
pub mod extensions_kubeblocks_io_v1alpha1;
#[cfg(feature = "external_secrets_io_v1alpha1")]
pub mod external_secrets_io_v1alpha1;
#[cfg(feature = "external_secrets_io_v1beta1")]
pub mod external_secrets_io_v1beta1;
#[cfg(feature = "externaldata_gatekeeper_sh_v1alpha1")]
pub mod externaldata_gatekeeper_sh_v1alpha1;
#[cfg(feature = "externaldata_gatekeeper_sh_v1beta1")]
pub mod externaldata_gatekeeper_sh_v1beta1;
#[cfg(feature = "externaldns_k8s_io_v1alpha1")]
pub mod externaldns_k8s_io_v1alpha1;
#[cfg(feature = "externaldns_nginx_org_v1")]
pub mod externaldns_nginx_org_v1;
#[cfg(feature = "flagger_app_v1beta1")]
pub mod flagger_app_v1beta1;
#[cfg(feature = "flink_apache_org_v1beta1")]
pub mod flink_apache_org_v1beta1;
#[cfg(feature = "flow_volcano_sh_v1alpha1")]
pub mod flow_volcano_sh_v1alpha1;
#[cfg(feature = "flows_netobserv_io_v1alpha1")]
pub mod flows_netobserv_io_v1alpha1;
#[cfg(feature = "flows_netobserv_io_v1beta1")]
pub mod flows_netobserv_io_v1beta1;
#[cfg(feature = "flows_netobserv_io_v1beta2")]
pub mod flows_netobserv_io_v1beta2;
#[cfg(feature = "flux_framework_org_v1alpha1")]
pub mod flux_framework_org_v1alpha1;
#[cfg(feature = "gateway_networking_k8s_io_v1")]
pub mod gateway_networking_k8s_io_v1;
#[cfg(feature = "gateway_networking_k8s_io_v1alpha2")]
pub mod gateway_networking_k8s_io_v1alpha2;
#[cfg(feature = "gateway_networking_k8s_io_v1beta1")]
pub mod gateway_networking_k8s_io_v1beta1;
#[cfg(feature = "gateway_nginx_org_v1alpha1")]
pub mod gateway_nginx_org_v1alpha1;
#[cfg(feature = "getambassador_io_v3alpha1")]
pub mod getambassador_io_v3alpha1;
#[cfg(feature = "gitops_hybrid_cloud_patterns_io_v1alpha1")]
pub mod gitops_hybrid_cloud_patterns_io_v1alpha1;
#[cfg(feature = "grafana_integreatly_org_v1beta1")]
pub mod grafana_integreatly_org_v1beta1;
#[cfg(feature = "hazelcast_com_v1alpha1")]
pub mod hazelcast_com_v1alpha1;
#[cfg(feature = "helm_toolkit_fluxcd_io_v2beta1")]
pub mod helm_toolkit_fluxcd_io_v2beta1;
#[cfg(feature = "hive_openshift_io_v1")]
pub mod hive_openshift_io_v1;
#[cfg(feature = "hiveinternal_openshift_io_v1alpha1")]
pub mod hiveinternal_openshift_io_v1alpha1;
#[cfg(feature = "hnc_x_k8s_io_v1alpha2")]
pub mod hnc_x_k8s_io_v1alpha2;
#[cfg(feature = "hyperfoil_io_v1alpha1")]
pub mod hyperfoil_io_v1alpha1;
#[cfg(feature = "hyperfoil_io_v1alpha2")]
pub mod hyperfoil_io_v1alpha2;
#[cfg(feature = "iam_services_k8s_aws_v1alpha1")]
pub mod iam_services_k8s_aws_v1alpha1;
#[cfg(feature = "ibmcloud_ibm_com_v1alpha1")]
pub mod ibmcloud_ibm_com_v1alpha1;
#[cfg(feature = "image_toolkit_fluxcd_io_v1beta1")]
pub mod image_toolkit_fluxcd_io_v1beta1;
#[cfg(feature = "image_toolkit_fluxcd_io_v1beta2")]
pub mod image_toolkit_fluxcd_io_v1beta2;
#[cfg(feature = "imaging_ingestion_alvearie_org_v1alpha1")]
pub mod imaging_ingestion_alvearie_org_v1alpha1;
#[cfg(feature = "inference_kubedl_io_v1alpha1")]
pub mod inference_kubedl_io_v1alpha1;
#[cfg(feature = "infinispan_org_v2alpha1")]
pub mod infinispan_org_v2alpha1;
#[cfg(feature = "infrastructure_cluster_x_k8s_io_v1alpha1")]
pub mod infrastructure_cluster_x_k8s_io_v1alpha1;
#[cfg(feature = "infrastructure_cluster_x_k8s_io_v1beta1")]
pub mod infrastructure_cluster_x_k8s_io_v1beta1;
#[cfg(feature = "infrastructure_cluster_x_k8s_io_v1beta2")]
pub mod infrastructure_cluster_x_k8s_io_v1beta2;
#[cfg(feature = "installation_mattermost_com_v1beta1")]
pub mod installation_mattermost_com_v1beta1;
#[cfg(feature = "integration_rock8s_com_v1beta1")]
pub mod integration_rock8s_com_v1beta1;
#[cfg(feature = "iot_eclipse_org_v1alpha1")]
pub mod iot_eclipse_org_v1alpha1;
#[cfg(feature = "ipam_cluster_x_k8s_io_v1alpha1")]
pub mod ipam_cluster_x_k8s_io_v1alpha1;
#[cfg(feature = "ipam_cluster_x_k8s_io_v1beta1")]
pub mod ipam_cluster_x_k8s_io_v1beta1;
#[cfg(feature = "jaegertracing_io_v1")]
pub mod jaegertracing_io_v1;
#[cfg(feature = "jobset_x_k8s_io_v1alpha2")]
pub mod jobset_x_k8s_io_v1alpha2;
#[cfg(feature = "k8gb_absa_oss_v1beta1")]
pub mod k8gb_absa_oss_v1beta1;
#[cfg(feature = "k8s_keycloak_org_v2alpha1")]
pub mod k8s_keycloak_org_v2alpha1;
#[cfg(feature = "k8s_nginx_org_v1")]
pub mod k8s_nginx_org_v1;
#[cfg(feature = "k8s_nginx_org_v1alpha1")]
pub mod k8s_nginx_org_v1alpha1;
#[cfg(feature = "k8s_otterize_com_v1alpha2")]
pub mod k8s_otterize_com_v1alpha2;
#[cfg(feature = "k8s_otterize_com_v1alpha3")]
pub mod k8s_otterize_com_v1alpha3;
#[cfg(feature = "kafka_strimzi_io_v1alpha1")]
pub mod kafka_strimzi_io_v1alpha1;
#[cfg(feature = "kafka_strimzi_io_v1beta1")]
pub mod kafka_strimzi_io_v1beta1;
#[cfg(feature = "kafka_strimzi_io_v1beta2")]
pub mod kafka_strimzi_io_v1beta2;
#[cfg(feature = "keda_sh_v1alpha1")]
pub mod keda_sh_v1alpha1;
#[cfg(feature = "keycloak_k8s_reddec_net_v1alpha1")]
pub mod keycloak_k8s_reddec_net_v1alpha1;
#[cfg(feature = "keycloak_org_v1alpha1")]
pub mod keycloak_org_v1alpha1;
#[cfg(feature = "kibana_k8s_elastic_co_v1")]
pub mod kibana_k8s_elastic_co_v1;
#[cfg(feature = "kibana_k8s_elastic_co_v1beta1")]
pub mod kibana_k8s_elastic_co_v1beta1;
#[cfg(feature = "kms_services_k8s_aws_v1alpha1")]
pub mod kms_services_k8s_aws_v1alpha1;
#[cfg(feature = "kubean_io_v1alpha1")]
pub mod kubean_io_v1alpha1;
#[cfg(feature = "kubevious_io_v1alpha1")]
pub mod kubevious_io_v1alpha1;
#[cfg(feature = "kueue_x_k8s_io_v1beta1")]
pub mod kueue_x_k8s_io_v1beta1;
#[cfg(feature = "kuma_io_v1alpha1")]
pub mod kuma_io_v1alpha1;
#[cfg(feature = "kustomize_toolkit_fluxcd_io_v1")]
pub mod kustomize_toolkit_fluxcd_io_v1;
#[cfg(feature = "kustomize_toolkit_fluxcd_io_v1beta1")]
pub mod kustomize_toolkit_fluxcd_io_v1beta1;
#[cfg(feature = "kustomize_toolkit_fluxcd_io_v1beta2")]
pub mod kustomize_toolkit_fluxcd_io_v1beta2;
#[cfg(feature = "kyverno_io_v1")]
pub mod kyverno_io_v1;
#[cfg(feature = "kyverno_io_v1alpha2")]
pub mod kyverno_io_v1alpha2;
#[cfg(feature = "kyverno_io_v1beta1")]
pub mod kyverno_io_v1beta1;
#[cfg(feature = "kyverno_io_v2alpha1")]
pub mod kyverno_io_v2alpha1;
#[cfg(feature = "kyverno_io_v2beta1")]
pub mod kyverno_io_v2beta1;
#[cfg(feature = "lambda_services_k8s_aws_v1alpha1")]
pub mod lambda_services_k8s_aws_v1alpha1;
#[cfg(feature = "lerentis_uploadfilter24_eu_v1beta4")]
pub mod lerentis_uploadfilter24_eu_v1beta4;
#[cfg(feature = "limitador_kuadrant_io_v1alpha1")]
pub mod limitador_kuadrant_io_v1alpha1;
#[cfg(feature = "litmuschaos_io_v1alpha1")]
pub mod litmuschaos_io_v1alpha1;
#[cfg(feature = "logging_extensions_banzaicloud_io_v1alpha1")]
pub mod logging_extensions_banzaicloud_io_v1alpha1;
#[cfg(feature = "logging_banzaicloud_io_v1alpha1")]
pub mod logging_banzaicloud_io_v1alpha1;
#[cfg(feature = "logging_banzaicloud_io_v1beta1")]
pub mod logging_banzaicloud_io_v1beta1;
#[cfg(feature = "loki_grafana_com_v1")]
pub mod loki_grafana_com_v1;
#[cfg(feature = "loki_grafana_com_v1beta1")]
pub mod loki_grafana_com_v1beta1;
#[cfg(feature = "longhorn_io_v1beta2")]
pub mod longhorn_io_v1beta2;
#[cfg(feature = "maps_k8s_elastic_co_v1alpha1")]
pub mod maps_k8s_elastic_co_v1alpha1;
#[cfg(feature = "mariadb_mmontes_io_v1alpha1")]
pub mod mariadb_mmontes_io_v1alpha1;
#[cfg(feature = "mattermost_com_v1alpha1")]
pub mod mattermost_com_v1alpha1;
#[cfg(feature = "metacontroller_k8s_io_v1alpha1")]
pub mod metacontroller_k8s_io_v1alpha1;
#[cfg(feature = "metal3_io_v1alpha1")]
pub mod metal3_io_v1alpha1;
#[cfg(feature = "minio_min_io_v2")]
pub mod minio_min_io_v2;
#[cfg(feature = "mirrors_kts_studio_v1alpha1")]
pub mod mirrors_kts_studio_v1alpha1;
#[cfg(feature = "mirrors_kts_studio_v1alpha2")]
pub mod mirrors_kts_studio_v1alpha2;
#[cfg(feature = "model_kubedl_io_v1alpha1")]
pub mod model_kubedl_io_v1alpha1;
#[cfg(feature = "monitoring_coreos_com_v1")]
pub mod monitoring_coreos_com_v1;
#[cfg(feature = "monitoring_coreos_com_v1alpha1")]
pub mod monitoring_coreos_com_v1alpha1;
#[cfg(feature = "monitoring_coreos_com_v1beta1")]
pub mod monitoring_coreos_com_v1beta1;
#[cfg(feature = "monocle_monocle_change_metrics_io_v1alpha1")]
pub mod monocle_monocle_change_metrics_io_v1alpha1;
#[cfg(feature = "mq_services_k8s_aws_v1alpha1")]
pub mod mq_services_k8s_aws_v1alpha1;
#[cfg(feature = "multicluster_crd_antrea_io_v1alpha1")]
pub mod multicluster_crd_antrea_io_v1alpha1;
#[cfg(feature = "multicluster_crd_antrea_io_v1alpha2")]
pub mod multicluster_crd_antrea_io_v1alpha2;
#[cfg(feature = "multicluster_x_k8s_io_v1alpha1")]
pub mod multicluster_x_k8s_io_v1alpha1;
#[cfg(feature = "mutations_gatekeeper_sh_v1")]
pub mod mutations_gatekeeper_sh_v1;
#[cfg(feature = "mutations_gatekeeper_sh_v1alpha1")]
pub mod mutations_gatekeeper_sh_v1alpha1;
#[cfg(feature = "mutations_gatekeeper_sh_v1beta1")]
pub mod mutations_gatekeeper_sh_v1beta1;
#[cfg(feature = "nativestor_alauda_io_v1")]
pub mod nativestor_alauda_io_v1;
#[cfg(feature = "networking_karmada_io_v1alpha1")]
pub mod networking_karmada_io_v1alpha1;
#[cfg(feature = "nfd_k8s_sigs_io_v1alpha1")]
pub mod nfd_k8s_sigs_io_v1alpha1;
#[cfg(feature = "nfd_kubernetes_io_v1")]
pub mod nfd_kubernetes_io_v1;
#[cfg(feature = "nfd_kubernetes_io_v1alpha1")]
pub mod nfd_kubernetes_io_v1alpha1;
#[cfg(feature = "nodeinfo_volcano_sh_v1alpha1")]
pub mod nodeinfo_volcano_sh_v1alpha1;
#[cfg(feature = "notebook_kubedl_io_v1alpha1")]
pub mod notebook_kubedl_io_v1alpha1;
#[cfg(feature = "notification_toolkit_fluxcd_io_v1")]
pub mod notification_toolkit_fluxcd_io_v1;
#[cfg(feature = "notification_toolkit_fluxcd_io_v1beta1")]
pub mod notification_toolkit_fluxcd_io_v1beta1;
#[cfg(feature = "notification_toolkit_fluxcd_io_v1beta2")]
pub mod notification_toolkit_fluxcd_io_v1beta2;
#[cfg(feature = "opensearchservice_services_k8s_aws_v1alpha1")]
pub mod opensearchservice_services_k8s_aws_v1alpha1;
#[cfg(feature = "opentelemetry_io_v1alpha1")]
pub mod opentelemetry_io_v1alpha1;
#[cfg(feature = "operations_kubeedge_io_v1alpha1")]
pub mod operations_kubeedge_io_v1alpha1;
#[cfg(feature = "operator_aquasec_com_v1alpha1")]
pub mod operator_aquasec_com_v1alpha1;
#[cfg(feature = "operator_authorino_kuadrant_io_v1beta1")]
pub mod operator_authorino_kuadrant_io_v1beta1;
#[cfg(feature = "operator_cluster_x_k8s_io_v1alpha1")]
pub mod operator_cluster_x_k8s_io_v1alpha1;
#[cfg(feature = "operator_cluster_x_k8s_io_v1alpha2")]
pub mod operator_cluster_x_k8s_io_v1alpha2;
#[cfg(feature = "operator_cryostat_io_v1beta1")]
pub mod operator_cryostat_io_v1beta1;
#[cfg(feature = "operator_open_cluster_management_io_v1")]
pub mod operator_open_cluster_management_io_v1;
#[cfg(feature = "operator_shipwright_io_v1alpha1")]
pub mod operator_shipwright_io_v1alpha1;
#[cfg(feature = "operator_tigera_io_v1")]
pub mod operator_tigera_io_v1;
#[cfg(feature = "operator_victoriametrics_com_v1beta1")]
pub mod operator_victoriametrics_com_v1beta1;
#[cfg(feature = "org_eclipse_che_v1")]
pub mod org_eclipse_che_v1;
#[cfg(feature = "org_eclipse_che_v2")]
pub mod org_eclipse_che_v2;
#[cfg(feature = "pkg_crossplane_io_v1")]
pub mod pkg_crossplane_io_v1;
#[cfg(feature = "pkg_crossplane_io_v1alpha1")]
pub mod pkg_crossplane_io_v1alpha1;
#[cfg(feature = "pkg_crossplane_io_v1beta1")]
pub mod pkg_crossplane_io_v1beta1;
#[cfg(feature = "policy_clusterpedia_io_v1alpha1")]
pub mod policy_clusterpedia_io_v1alpha1;
#[cfg(feature = "policy_karmada_io_v1alpha1")]
pub mod policy_karmada_io_v1alpha1;
#[cfg(feature = "postgres_operator_crunchydata_com_v1beta1")]
pub mod postgres_operator_crunchydata_com_v1beta1;
#[cfg(feature = "postgresql_cnpg_io_v1")]
pub mod postgresql_cnpg_io_v1;
#[cfg(feature = "prometheusservice_services_k8s_aws_v1alpha1")]
pub mod prometheusservice_services_k8s_aws_v1alpha1;
#[cfg(feature = "quay_redhat_com_v1")]
pub mod quay_redhat_com_v1;
#[cfg(feature = "ray_io_v1")]
pub mod ray_io_v1;
#[cfg(feature = "ray_io_v1alpha1")]
pub mod ray_io_v1alpha1;
#[cfg(feature = "rds_services_k8s_aws_v1alpha1")]
pub mod rds_services_k8s_aws_v1alpha1;
#[cfg(feature = "redhatcop_redhat_io_v1alpha1")]
pub mod redhatcop_redhat_io_v1alpha1;
#[cfg(feature = "registry_apicur_io_v1")]
pub mod registry_apicur_io_v1;
#[cfg(feature = "registry_devfile_io_v1alpha1")]
pub mod registry_devfile_io_v1alpha1;
#[cfg(feature = "reliablesyncs_kubeedge_io_v1alpha1")]
pub mod reliablesyncs_kubeedge_io_v1alpha1;
#[cfg(feature = "repo_manager_pulpproject_org_v1beta2")]
pub mod repo_manager_pulpproject_org_v1beta2;
#[cfg(feature = "resources_teleport_dev_v1")]
pub mod resources_teleport_dev_v1;
#[cfg(feature = "resources_teleport_dev_v2")]
pub mod resources_teleport_dev_v2;
#[cfg(feature = "resources_teleport_dev_v3")]
pub mod resources_teleport_dev_v3;
#[cfg(feature = "rocketmq_apache_org_v1alpha1")]
pub mod rocketmq_apache_org_v1alpha1;
#[cfg(feature = "rules_kubeedge_io_v1")]
pub mod rules_kubeedge_io_v1;
#[cfg(feature = "runtime_cluster_x_k8s_io_v1alpha1")]
pub mod runtime_cluster_x_k8s_io_v1alpha1;
#[cfg(feature = "s3_services_k8s_aws_v1alpha1")]
pub mod s3_services_k8s_aws_v1alpha1;
#[cfg(feature = "sagemaker_services_k8s_aws_v1alpha1")]
pub mod sagemaker_services_k8s_aws_v1alpha1;
#[cfg(feature = "scheduling_koordinator_sh_v1alpha1")]
pub mod scheduling_koordinator_sh_v1alpha1;
#[cfg(feature = "scheduling_sigs_k8s_io_v1alpha1")]
pub mod scheduling_sigs_k8s_io_v1alpha1;
#[cfg(feature = "scheduling_volcano_sh_v1beta1")]
pub mod scheduling_volcano_sh_v1beta1;
#[cfg(feature = "schemas_schemahero_io_v1alpha4")]
pub mod schemas_schemahero_io_v1alpha4;
#[cfg(feature = "scylla_scylladb_com_v1")]
pub mod scylla_scylladb_com_v1;
#[cfg(feature = "scylla_scylladb_com_v1alpha1")]
pub mod scylla_scylladb_com_v1alpha1;
#[cfg(feature = "secretgenerator_mittwald_de_v1alpha1")]
pub mod secretgenerator_mittwald_de_v1alpha1;
#[cfg(feature = "secrets_crossplane_io_v1alpha1")]
pub mod secrets_crossplane_io_v1alpha1;
#[cfg(feature = "secscan_quay_redhat_com_v1alpha1")]
pub mod secscan_quay_redhat_com_v1alpha1;
#[cfg(feature = "security_profiles_operator_x_k8s_io_v1alpha1")]
pub mod security_profiles_operator_x_k8s_io_v1alpha1;
#[cfg(feature = "security_profiles_operator_x_k8s_io_v1alpha2")]
pub mod security_profiles_operator_x_k8s_io_v1alpha2;
#[cfg(feature = "security_profiles_operator_x_k8s_io_v1beta1")]
pub mod security_profiles_operator_x_k8s_io_v1beta1;
#[cfg(feature = "servicebinding_io_v1alpha3")]
pub mod servicebinding_io_v1alpha3;
#[cfg(feature = "servicebinding_io_v1beta1")]
pub mod servicebinding_io_v1beta1;
#[cfg(feature = "services_k8s_aws_v1alpha1")]
pub mod services_k8s_aws_v1alpha1;
#[cfg(feature = "serving_kubedl_io_v1alpha1")]
pub mod serving_kubedl_io_v1alpha1;
#[cfg(feature = "sfn_services_k8s_aws_v1alpha1")]
pub mod sfn_services_k8s_aws_v1alpha1;
#[cfg(feature = "site_superedge_io_v1alpha1")]
pub mod site_superedge_io_v1alpha1;
#[cfg(feature = "slo_koordinator_sh_v1alpha1")]
pub mod slo_koordinator_sh_v1alpha1;
#[cfg(feature = "sonataflow_org_v1alpha08")]
pub mod sonataflow_org_v1alpha08;
#[cfg(feature = "source_toolkit_fluxcd_io_v1beta1")]
pub mod source_toolkit_fluxcd_io_v1beta1;
#[cfg(feature = "source_toolkit_fluxcd_io_v1beta2")]
pub mod source_toolkit_fluxcd_io_v1beta2;
#[cfg(feature = "sparkoperator_k8s_io_v1beta2")]
pub mod sparkoperator_k8s_io_v1beta2;
#[cfg(feature = "status_gatekeeper_sh_v1beta1")]
pub mod status_gatekeeper_sh_v1beta1;
#[cfg(feature = "storage_kubeblocks_io_v1alpha1")]
pub mod storage_kubeblocks_io_v1alpha1;
#[cfg(feature = "sts_min_io_v1alpha1")]
pub mod sts_min_io_v1alpha1;
#[cfg(feature = "stunner_l7mp_io_v1alpha1")]
pub mod stunner_l7mp_io_v1alpha1;
#[cfg(feature = "submariner_io_v1alpha1")]
pub mod submariner_io_v1alpha1;
#[cfg(feature = "templates_gatekeeper_sh_v1")]
pub mod templates_gatekeeper_sh_v1;
#[cfg(feature = "templates_gatekeeper_sh_v1alpha1")]
pub mod templates_gatekeeper_sh_v1alpha1;
#[cfg(feature = "templates_gatekeeper_sh_v1beta1")]
pub mod templates_gatekeeper_sh_v1beta1;
#[cfg(feature = "tests_testkube_io_v1")]
pub mod tests_testkube_io_v1;
#[cfg(feature = "tests_testkube_io_v2")]
pub mod tests_testkube_io_v2;
#[cfg(feature = "tests_testkube_io_v3")]
pub mod tests_testkube_io_v3;
#[cfg(feature = "topology_node_k8s_io_v1alpha1")]
pub mod topology_node_k8s_io_v1alpha1;
#[cfg(feature = "topolvm_cybozu_com_v1")]
pub mod topolvm_cybozu_com_v1;
#[cfg(feature = "topolvm_cybozu_com_v2")]
pub mod topolvm_cybozu_com_v2;
#[cfg(feature = "traefik_io_v1alpha1")]
pub mod traefik_io_v1alpha1;
#[cfg(feature = "training_kubedl_io_v1alpha1")]
pub mod training_kubedl_io_v1alpha1;
#[cfg(feature = "virt_virtink_smartx_com_v1alpha1")]
pub mod virt_virtink_smartx_com_v1alpha1;
#[cfg(feature = "wgpolicyk8s_io_v1alpha1")]
pub mod wgpolicyk8s_io_v1alpha1;
#[cfg(feature = "wgpolicyk8s_io_v1alpha2")]
pub mod wgpolicyk8s_io_v1alpha2;
#[cfg(feature = "wgpolicyk8s_io_v1beta1")]
pub mod wgpolicyk8s_io_v1beta1;
#[cfg(feature = "wildfly_org_v1alpha1")]
pub mod wildfly_org_v1alpha1;
#[cfg(feature = "work_karmada_io_v1alpha1")]
pub mod work_karmada_io_v1alpha1;
#[cfg(feature = "work_karmada_io_v1alpha2")]
pub mod work_karmada_io_v1alpha2;
#[cfg(feature = "workloads_kubeblocks_io_v1alpha1")]
pub mod workloads_kubeblocks_io_v1alpha1;
