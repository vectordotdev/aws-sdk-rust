// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePackagingConfiguration`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cmaf_package(CmafPackage)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::cmaf_package) / [`set_cmaf_package(Option<CmafPackage>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_cmaf_package): A CMAF packaging configuration.
    ///   - [`dash_package(DashPackage)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::dash_package) / [`set_dash_package(Option<DashPackage>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_dash_package): A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.
    ///   - [`hls_package(HlsPackage)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::hls_package) / [`set_hls_package(Option<HlsPackage>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_hls_package): An HTTP Live Streaming (HLS) packaging configuration.
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_id): The ID of the PackagingConfiguration.
    ///   - [`mss_package(MssPackage)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::mss_package) / [`set_mss_package(Option<MssPackage>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_mss_package): A Microsoft Smooth Streaming (MSS) PackagingConfiguration.
    ///   - [`packaging_group_id(impl ::std::convert::Into<String>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::packaging_group_id) / [`set_packaging_group_id(Option<String>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_packaging_group_id): The ID of a PackagingGroup.
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::set_tags): A collection of tags associated with a resource
    /// - On success, responds with [`CreatePackagingConfigurationOutput`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::arn): The ARN of the PackagingConfiguration.
    ///   - [`cmaf_package(Option<CmafPackage>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::cmaf_package): A CMAF packaging configuration.
    ///   - [`created_at(Option<String>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::created_at): The time the PackagingConfiguration was created.
    ///   - [`dash_package(Option<DashPackage>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::dash_package): A Dynamic Adaptive Streaming over HTTP (DASH) packaging configuration.
    ///   - [`hls_package(Option<HlsPackage>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::hls_package): An HTTP Live Streaming (HLS) packaging configuration.
    ///   - [`id(Option<String>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::id): The ID of the PackagingConfiguration.
    ///   - [`mss_package(Option<MssPackage>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::mss_package): A Microsoft Smooth Streaming (MSS) PackagingConfiguration.
    ///   - [`packaging_group_id(Option<String>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::packaging_group_id): The ID of a PackagingGroup.
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationOutput::tags): A collection of tags associated with a resource
    /// - On failure, responds with [`SdkError<CreatePackagingConfigurationError>`](crate::operation::create_packaging_configuration::CreatePackagingConfigurationError)
    pub fn create_packaging_configuration(&self) -> crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder{
        crate::operation::create_packaging_configuration::builders::CreatePackagingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
