// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that identifies an item.</p>
/// <p>The and APIs return a list of <code>PredictedItem</code>s.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PredictedItem {
    /// <p>The recommended item ID.</p>
    #[doc(hidden)]
    pub item_id: ::std::option::Option<::std::string::String>,
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    #[doc(hidden)]
    pub score: ::std::option::Option<f64>,
    /// <p>The name of the promotion that included the predicted item.</p>
    #[doc(hidden)]
    pub promotion_name: ::std::option::Option<::std::string::String>,
}
impl PredictedItem {
    /// <p>The recommended item ID.</p>
    pub fn item_id(&self) -> ::std::option::Option<&str> {
        self.item_id.as_deref()
    }
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    pub fn score(&self) -> ::std::option::Option<f64> {
        self.score
    }
    /// <p>The name of the promotion that included the predicted item.</p>
    pub fn promotion_name(&self) -> ::std::option::Option<&str> {
        self.promotion_name.as_deref()
    }
}
impl PredictedItem {
    /// Creates a new builder-style object to manufacture [`PredictedItem`](crate::types::PredictedItem).
    pub fn builder() -> crate::types::builders::PredictedItemBuilder {
        crate::types::builders::PredictedItemBuilder::default()
    }
}

/// A builder for [`PredictedItem`](crate::types::PredictedItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PredictedItemBuilder {
    pub(crate) item_id: ::std::option::Option<::std::string::String>,
    pub(crate) score: ::std::option::Option<f64>,
    pub(crate) promotion_name: ::std::option::Option<::std::string::String>,
}
impl PredictedItemBuilder {
    /// <p>The recommended item ID.</p>
    pub fn item_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.item_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The recommended item ID.</p>
    pub fn set_item_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.item_id = input;
        self
    }
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    pub fn score(mut self, input: f64) -> Self {
        self.score = ::std::option::Option::Some(input);
        self
    }
    /// <p>A numeric representation of the model's certainty that the item will be the next user selection. For more information on scoring logic, see <code>how-scores-work</code>.</p>
    pub fn set_score(mut self, input: ::std::option::Option<f64>) -> Self {
        self.score = input;
        self
    }
    /// <p>The name of the promotion that included the predicted item.</p>
    pub fn promotion_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.promotion_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the promotion that included the predicted item.</p>
    pub fn set_promotion_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.promotion_name = input;
        self
    }
    /// Consumes the builder and constructs a [`PredictedItem`](crate::types::PredictedItem).
    pub fn build(self) -> crate::types::PredictedItem {
        crate::types::PredictedItem {
            item_id: self.item_id,
            score: self.score,
            promotion_name: self.promotion_name,
        }
    }
}
