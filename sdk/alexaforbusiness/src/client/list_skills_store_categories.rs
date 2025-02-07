// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSkillsStoreCategories`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::set_next_token): <p>The tokens used for pagination.</p>
    ///   - [`max_results(i32)`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::set_max_results): <p>The maximum number of categories returned, per paginated calls.</p>
    /// - On success, responds with [`ListSkillsStoreCategoriesOutput`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput) with field(s):
    ///   - [`category_list(Option<Vec<Category>>)`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput::category_list): <p>The list of categories.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesOutput::next_token): <p>The tokens used for pagination.</p>
    /// - On failure, responds with [`SdkError<ListSkillsStoreCategoriesError>`](crate::operation::list_skills_store_categories::ListSkillsStoreCategoriesError)
    pub fn list_skills_store_categories(&self) -> crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder{
        crate::operation::list_skills_store_categories::builders::ListSkillsStoreCategoriesFluentBuilder::new(self.handle.clone())
    }
}
