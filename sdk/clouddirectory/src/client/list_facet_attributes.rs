// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFacetAttributes`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_arn(impl ::std::convert::Into<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::schema_arn) / [`set_schema_arn(Option<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::set_schema_arn): <p>The ARN of the schema where the facet resides.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::set_name): <p>The name of the facet whose attributes will be retrieved.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::set_next_token): <p>The pagination token.</p>
    ///   - [`max_results(i32)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::set_max_results): <p>The maximum number of results to retrieve.</p>
    /// - On success, responds with [`ListFacetAttributesOutput`](crate::operation::list_facet_attributes::ListFacetAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<FacetAttribute>>)`](crate::operation::list_facet_attributes::ListFacetAttributesOutput::attributes): <p>The attributes attached to the facet.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_facet_attributes::ListFacetAttributesOutput::next_token): <p>The pagination token.</p>
    /// - On failure, responds with [`SdkError<ListFacetAttributesError>`](crate::operation::list_facet_attributes::ListFacetAttributesError)
    pub fn list_facet_attributes(
        &self,
    ) -> crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder {
        crate::operation::list_facet_attributes::builders::ListFacetAttributesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
