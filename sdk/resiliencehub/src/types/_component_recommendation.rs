// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines recommendations for an Resilience Hub Application Component, returned as an object. This object contains component names, configuration recommendations, and recommendation statuses.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ComponentRecommendation {
    /// <p>The name of the Application Component.</p>
    #[doc(hidden)]
    pub app_component_name: ::std::option::Option<::std::string::String>,
    /// <p>The recommendation status.</p>
    #[doc(hidden)]
    pub recommendation_status: ::std::option::Option<crate::types::RecommendationComplianceStatus>,
    /// <p>The list of recommendations.</p>
    #[doc(hidden)]
    pub config_recommendations:
        ::std::option::Option<::std::vec::Vec<crate::types::ConfigRecommendation>>,
}
impl ComponentRecommendation {
    /// <p>The name of the Application Component.</p>
    pub fn app_component_name(&self) -> ::std::option::Option<&str> {
        self.app_component_name.as_deref()
    }
    /// <p>The recommendation status.</p>
    pub fn recommendation_status(
        &self,
    ) -> ::std::option::Option<&crate::types::RecommendationComplianceStatus> {
        self.recommendation_status.as_ref()
    }
    /// <p>The list of recommendations.</p>
    pub fn config_recommendations(
        &self,
    ) -> ::std::option::Option<&[crate::types::ConfigRecommendation]> {
        self.config_recommendations.as_deref()
    }
}
impl ComponentRecommendation {
    /// Creates a new builder-style object to manufacture [`ComponentRecommendation`](crate::types::ComponentRecommendation).
    pub fn builder() -> crate::types::builders::ComponentRecommendationBuilder {
        crate::types::builders::ComponentRecommendationBuilder::default()
    }
}

/// A builder for [`ComponentRecommendation`](crate::types::ComponentRecommendation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ComponentRecommendationBuilder {
    pub(crate) app_component_name: ::std::option::Option<::std::string::String>,
    pub(crate) recommendation_status:
        ::std::option::Option<crate::types::RecommendationComplianceStatus>,
    pub(crate) config_recommendations:
        ::std::option::Option<::std::vec::Vec<crate::types::ConfigRecommendation>>,
}
impl ComponentRecommendationBuilder {
    /// <p>The name of the Application Component.</p>
    pub fn app_component_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_component_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Application Component.</p>
    pub fn set_app_component_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_component_name = input;
        self
    }
    /// <p>The recommendation status.</p>
    pub fn recommendation_status(
        mut self,
        input: crate::types::RecommendationComplianceStatus,
    ) -> Self {
        self.recommendation_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The recommendation status.</p>
    pub fn set_recommendation_status(
        mut self,
        input: ::std::option::Option<crate::types::RecommendationComplianceStatus>,
    ) -> Self {
        self.recommendation_status = input;
        self
    }
    /// Appends an item to `config_recommendations`.
    ///
    /// To override the contents of this collection use [`set_config_recommendations`](Self::set_config_recommendations).
    ///
    /// <p>The list of recommendations.</p>
    pub fn config_recommendations(mut self, input: crate::types::ConfigRecommendation) -> Self {
        let mut v = self.config_recommendations.unwrap_or_default();
        v.push(input);
        self.config_recommendations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of recommendations.</p>
    pub fn set_config_recommendations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigRecommendation>>,
    ) -> Self {
        self.config_recommendations = input;
        self
    }
    /// Consumes the builder and constructs a [`ComponentRecommendation`](crate::types::ComponentRecommendation).
    pub fn build(self) -> crate::types::ComponentRecommendation {
        crate::types::ComponentRecommendation {
            app_component_name: self.app_component_name,
            recommendation_status: self.recommendation_status,
            config_recommendations: self.config_recommendations,
        }
    }
}
