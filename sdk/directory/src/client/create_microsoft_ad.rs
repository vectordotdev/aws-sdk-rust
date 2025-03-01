// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateMicrosoftAD`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_name): <p>The fully qualified domain name for the Managed Microsoft AD directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>
    ///   - [`short_name(impl ::std::convert::Into<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::short_name) / [`set_short_name(Option<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_short_name): <p>The NetBIOS name for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>
    ///   - [`password(impl ::std::convert::Into<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_password): <p>The password for the default administrative user named <code>Admin</code>.</p>  <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_description): <p>A description for the directory. This label will appear on the Amazon Web Services console <code>Directory Details</code> page after the directory is created.</p>
    ///   - [`vpc_settings(DirectoryVpcSettings)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::vpc_settings) / [`set_vpc_settings(Option<DirectoryVpcSettings>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_vpc_settings): <p>Contains VPC information for the <code>CreateDirectory</code> or <code>CreateMicrosoftAD</code> operation.</p>
    ///   - [`edition(DirectoryEdition)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::edition) / [`set_edition(Option<DirectoryEdition>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_edition): <p>Managed Microsoft AD is available in two editions: <code>Standard</code> and <code>Enterprise</code>. <code>Enterprise</code> is the default.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::set_tags): <p>The tags to be assigned to the Managed Microsoft AD directory.</p>
    /// - On success, responds with [`CreateMicrosoftAdOutput`](crate::operation::create_microsoft_ad::CreateMicrosoftAdOutput) with field(s):
    ///   - [`directory_id(Option<String>)`](crate::operation::create_microsoft_ad::CreateMicrosoftAdOutput::directory_id): <p>The identifier of the directory that was created.</p>
    /// - On failure, responds with [`SdkError<CreateMicrosoftADError>`](crate::operation::create_microsoft_ad::CreateMicrosoftADError)
    pub fn create_microsoft_ad(
        &self,
    ) -> crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder {
        crate::operation::create_microsoft_ad::builders::CreateMicrosoftADFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
