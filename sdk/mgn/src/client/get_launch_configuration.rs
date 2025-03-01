// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLaunchConfiguration`](crate::operation::get_launch_configuration::builders::GetLaunchConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_server_id(impl ::std::convert::Into<String>)`](crate::operation::get_launch_configuration::builders::GetLaunchConfigurationFluentBuilder::source_server_id) / [`set_source_server_id(Option<String>)`](crate::operation::get_launch_configuration::builders::GetLaunchConfigurationFluentBuilder::set_source_server_id): <p>Request to get Launch Configuration information by Source Server ID.</p>
    /// - On success, responds with [`GetLaunchConfigurationOutput`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput) with field(s):
    ///   - [`source_server_id(Option<String>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::source_server_id): <p>Launch configuration Source Server ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::name): <p>Launch configuration name.</p>
    ///   - [`ec2_launch_template_id(Option<String>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::ec2_launch_template_id): <p>Launch configuration EC2 Launch template ID.</p>
    ///   - [`launch_disposition(Option<LaunchDisposition>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::launch_disposition): <p>Launch disposition for launch configuration.</p>
    ///   - [`target_instance_type_right_sizing_method(Option<TargetInstanceTypeRightSizingMethod>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::target_instance_type_right_sizing_method): <p>Launch configuration Target instance type right sizing method.</p>
    ///   - [`copy_private_ip(Option<bool>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::copy_private_ip): <p>Copy Private IP during Launch Configuration.</p>
    ///   - [`copy_tags(Option<bool>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::copy_tags): <p>Copy Tags during Launch Configuration.</p>
    ///   - [`licensing(Option<Licensing>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::licensing): <p>Launch configuration OS licensing.</p>
    ///   - [`boot_mode(Option<BootMode>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::boot_mode): <p>Launch configuration boot mode.</p>
    ///   - [`post_launch_actions(Option<PostLaunchActions>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::post_launch_actions): <p>Post Launch Actions to executed on the Test or Cutover instance.</p>
    ///   - [`enable_map_auto_tagging(Option<bool>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::enable_map_auto_tagging): <p>Enable map auto tagging.</p>
    ///   - [`map_auto_tagging_mpe_id(Option<String>)`](crate::operation::get_launch_configuration::GetLaunchConfigurationOutput::map_auto_tagging_mpe_id): <p>Map auto tagging MPE ID.</p>
    /// - On failure, responds with [`SdkError<GetLaunchConfigurationError>`](crate::operation::get_launch_configuration::GetLaunchConfigurationError)
    pub fn get_launch_configuration(
        &self,
    ) -> crate::operation::get_launch_configuration::builders::GetLaunchConfigurationFluentBuilder
    {
        crate::operation::get_launch_configuration::builders::GetLaunchConfigurationFluentBuilder::new(self.handle.clone())
    }
}
