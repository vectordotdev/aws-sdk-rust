// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetReplicationConfiguration`](crate::operation::get_replication_configuration::builders::GetReplicationConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_server_id(impl ::std::convert::Into<String>)`](crate::operation::get_replication_configuration::builders::GetReplicationConfigurationFluentBuilder::source_server_id) / [`set_source_server_id(Option<String>)`](crate::operation::get_replication_configuration::builders::GetReplicationConfigurationFluentBuilder::set_source_server_id): <p>Request to get Replication Configuration by Source Server ID.</p>
    /// - On success, responds with [`GetReplicationConfigurationOutput`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput) with field(s):
    ///   - [`source_server_id(Option<String>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::source_server_id): <p>Replication Configuration Source Server ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::name): <p>Replication Configuration name.</p>
    ///   - [`staging_area_subnet_id(Option<String>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::staging_area_subnet_id): <p>Replication Configuration Staging Area subnet ID.</p>
    ///   - [`associate_default_security_group(Option<bool>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::associate_default_security_group): <p>Replication Configuration associate default Application Migration Service Security Group.</p>
    ///   - [`replication_servers_security_groups_i_ds(Option<Vec<String>>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::replication_servers_security_groups_i_ds): <p>Replication Configuration Replication Server Security Group IDs.</p>
    ///   - [`replication_server_instance_type(Option<String>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::replication_server_instance_type): <p>Replication Configuration Replication Server instance type.</p>
    ///   - [`use_dedicated_replication_server(Option<bool>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::use_dedicated_replication_server): <p>Replication Configuration use Dedicated Replication Server.</p>
    ///   - [`default_large_staging_disk_type(Option<ReplicationConfigurationDefaultLargeStagingDiskType>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::default_large_staging_disk_type): <p>Replication Configuration use default large Staging Disks.</p>
    ///   - [`replicated_disks(Option<Vec<ReplicationConfigurationReplicatedDisk>>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::replicated_disks): <p>Replication Configuration replicated disks.</p>
    ///   - [`ebs_encryption(Option<ReplicationConfigurationEbsEncryption>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::ebs_encryption): <p>Replication Configuration EBS encryption.</p>
    ///   - [`ebs_encryption_key_arn(Option<String>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::ebs_encryption_key_arn): <p>Replication Configuration EBS encryption key ARN.</p>
    ///   - [`bandwidth_throttling(i64)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::bandwidth_throttling): <p>Replication Configuration set bandwidth throttling.</p>
    ///   - [`data_plane_routing(Option<ReplicationConfigurationDataPlaneRouting>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::data_plane_routing): <p>Replication Configuration data plane routing.</p>
    ///   - [`create_public_ip(Option<bool>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::create_public_ip): <p>Replication Configuration create Public IP.</p>
    ///   - [`staging_area_tags(Option<HashMap<String, String>>)`](crate::operation::get_replication_configuration::GetReplicationConfigurationOutput::staging_area_tags): <p>Replication Configuration Staging Area tags.</p>
    /// - On failure, responds with [`SdkError<GetReplicationConfigurationError>`](crate::operation::get_replication_configuration::GetReplicationConfigurationError)
    pub fn get_replication_configuration(&self) -> crate::operation::get_replication_configuration::builders::GetReplicationConfigurationFluentBuilder{
        crate::operation::get_replication_configuration::builders::GetReplicationConfigurationFluentBuilder::new(self.handle.clone())
    }
}
