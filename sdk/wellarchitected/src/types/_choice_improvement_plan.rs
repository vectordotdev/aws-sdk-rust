// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The choice level improvement plan.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ChoiceImprovementPlan {
    /// <p>The ID of a choice.</p>
    #[doc(hidden)]
    pub choice_id: ::std::option::Option<::std::string::String>,
    /// <p>The display text for the improvement plan.</p>
    #[doc(hidden)]
    pub display_text: ::std::option::Option<::std::string::String>,
    /// <p>The improvement plan URL for a question in an Amazon Web Services official lenses.</p>
    /// <p>This value is only available if the question has been answered.</p>
    /// <p>This value does not apply to custom lenses.</p>
    #[doc(hidden)]
    pub improvement_plan_url: ::std::option::Option<::std::string::String>,
}
impl ChoiceImprovementPlan {
    /// <p>The ID of a choice.</p>
    pub fn choice_id(&self) -> ::std::option::Option<&str> {
        self.choice_id.as_deref()
    }
    /// <p>The display text for the improvement plan.</p>
    pub fn display_text(&self) -> ::std::option::Option<&str> {
        self.display_text.as_deref()
    }
    /// <p>The improvement plan URL for a question in an Amazon Web Services official lenses.</p>
    /// <p>This value is only available if the question has been answered.</p>
    /// <p>This value does not apply to custom lenses.</p>
    pub fn improvement_plan_url(&self) -> ::std::option::Option<&str> {
        self.improvement_plan_url.as_deref()
    }
}
impl ChoiceImprovementPlan {
    /// Creates a new builder-style object to manufacture [`ChoiceImprovementPlan`](crate::types::ChoiceImprovementPlan).
    pub fn builder() -> crate::types::builders::ChoiceImprovementPlanBuilder {
        crate::types::builders::ChoiceImprovementPlanBuilder::default()
    }
}

/// A builder for [`ChoiceImprovementPlan`](crate::types::ChoiceImprovementPlan).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ChoiceImprovementPlanBuilder {
    pub(crate) choice_id: ::std::option::Option<::std::string::String>,
    pub(crate) display_text: ::std::option::Option<::std::string::String>,
    pub(crate) improvement_plan_url: ::std::option::Option<::std::string::String>,
}
impl ChoiceImprovementPlanBuilder {
    /// <p>The ID of a choice.</p>
    pub fn choice_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.choice_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a choice.</p>
    pub fn set_choice_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.choice_id = input;
        self
    }
    /// <p>The display text for the improvement plan.</p>
    pub fn display_text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display text for the improvement plan.</p>
    pub fn set_display_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_text = input;
        self
    }
    /// <p>The improvement plan URL for a question in an Amazon Web Services official lenses.</p>
    /// <p>This value is only available if the question has been answered.</p>
    /// <p>This value does not apply to custom lenses.</p>
    pub fn improvement_plan_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.improvement_plan_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The improvement plan URL for a question in an Amazon Web Services official lenses.</p>
    /// <p>This value is only available if the question has been answered.</p>
    /// <p>This value does not apply to custom lenses.</p>
    pub fn set_improvement_plan_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.improvement_plan_url = input;
        self
    }
    /// Consumes the builder and constructs a [`ChoiceImprovementPlan`](crate::types::ChoiceImprovementPlan).
    pub fn build(self) -> crate::types::ChoiceImprovementPlan {
        crate::types::ChoiceImprovementPlan {
            choice_id: self.choice_id,
            display_text: self.display_text,
            improvement_plan_url: self.improvement_plan_url,
        }
    }
}
