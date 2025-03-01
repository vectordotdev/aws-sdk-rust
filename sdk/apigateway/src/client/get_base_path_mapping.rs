// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBasePathMapping`](crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder::set_domain_name): <p>The domain name of the BasePathMapping resource to be described.</p>
    ///   - [`base_path(impl ::std::convert::Into<String>)`](crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder::base_path) / [`set_base_path(Option<String>)`](crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder::set_base_path): <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Specify '(none)' if you do not want callers to specify any base path name after the domain name.</p>
    /// - On success, responds with [`GetBasePathMappingOutput`](crate::operation::get_base_path_mapping::GetBasePathMappingOutput) with field(s):
    ///   - [`base_path(Option<String>)`](crate::operation::get_base_path_mapping::GetBasePathMappingOutput::base_path): <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p>
    ///   - [`rest_api_id(Option<String>)`](crate::operation::get_base_path_mapping::GetBasePathMappingOutput::rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`stage(Option<String>)`](crate::operation::get_base_path_mapping::GetBasePathMappingOutput::stage): <p>The name of the associated stage.</p>
    /// - On failure, responds with [`SdkError<GetBasePathMappingError>`](crate::operation::get_base_path_mapping::GetBasePathMappingError)
    pub fn get_base_path_mapping(
        &self,
    ) -> crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder {
        crate::operation::get_base_path_mapping::builders::GetBasePathMappingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
