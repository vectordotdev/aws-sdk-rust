// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The top movers and bottom movers computation setup.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TopBottomMoversComputation {
    /// <p>The ID for a computation.</p>
    #[doc(hidden)]
    pub computation_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of a computation.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The time field that is used in a computation.</p>
    #[doc(hidden)]
    pub time: ::std::option::Option<crate::types::DimensionField>,
    /// <p>The category field that is used in a computation.</p>
    #[doc(hidden)]
    pub category: ::std::option::Option<crate::types::DimensionField>,
    /// <p>The value field that is used in a computation.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<crate::types::MeasureField>,
    /// <p>The mover size setup of the top and bottom movers computation.</p>
    #[doc(hidden)]
    pub mover_size: i32,
    /// <p>The sort order setup of the top and bottom movers computation.</p>
    #[doc(hidden)]
    pub sort_order: ::std::option::Option<crate::types::TopBottomSortOrder>,
    /// <p>The computation type. Choose from the following options:</p>
    /// <ul>
    /// <li> <p>TOP: Top movers computation.</p> </li>
    /// <li> <p>BOTTOM: Bottom movers computation.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::TopBottomComputationType>,
}
impl TopBottomMoversComputation {
    /// <p>The ID for a computation.</p>
    pub fn computation_id(&self) -> ::std::option::Option<&str> {
        self.computation_id.as_deref()
    }
    /// <p>The name of a computation.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The time field that is used in a computation.</p>
    pub fn time(&self) -> ::std::option::Option<&crate::types::DimensionField> {
        self.time.as_ref()
    }
    /// <p>The category field that is used in a computation.</p>
    pub fn category(&self) -> ::std::option::Option<&crate::types::DimensionField> {
        self.category.as_ref()
    }
    /// <p>The value field that is used in a computation.</p>
    pub fn value(&self) -> ::std::option::Option<&crate::types::MeasureField> {
        self.value.as_ref()
    }
    /// <p>The mover size setup of the top and bottom movers computation.</p>
    pub fn mover_size(&self) -> i32 {
        self.mover_size
    }
    /// <p>The sort order setup of the top and bottom movers computation.</p>
    pub fn sort_order(&self) -> ::std::option::Option<&crate::types::TopBottomSortOrder> {
        self.sort_order.as_ref()
    }
    /// <p>The computation type. Choose from the following options:</p>
    /// <ul>
    /// <li> <p>TOP: Top movers computation.</p> </li>
    /// <li> <p>BOTTOM: Bottom movers computation.</p> </li>
    /// </ul>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::TopBottomComputationType> {
        self.r#type.as_ref()
    }
}
impl TopBottomMoversComputation {
    /// Creates a new builder-style object to manufacture [`TopBottomMoversComputation`](crate::types::TopBottomMoversComputation).
    pub fn builder() -> crate::types::builders::TopBottomMoversComputationBuilder {
        crate::types::builders::TopBottomMoversComputationBuilder::default()
    }
}

/// A builder for [`TopBottomMoversComputation`](crate::types::TopBottomMoversComputation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TopBottomMoversComputationBuilder {
    pub(crate) computation_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) time: ::std::option::Option<crate::types::DimensionField>,
    pub(crate) category: ::std::option::Option<crate::types::DimensionField>,
    pub(crate) value: ::std::option::Option<crate::types::MeasureField>,
    pub(crate) mover_size: ::std::option::Option<i32>,
    pub(crate) sort_order: ::std::option::Option<crate::types::TopBottomSortOrder>,
    pub(crate) r#type: ::std::option::Option<crate::types::TopBottomComputationType>,
}
impl TopBottomMoversComputationBuilder {
    /// <p>The ID for a computation.</p>
    pub fn computation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.computation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID for a computation.</p>
    pub fn set_computation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.computation_id = input;
        self
    }
    /// <p>The name of a computation.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a computation.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The time field that is used in a computation.</p>
    pub fn time(mut self, input: crate::types::DimensionField) -> Self {
        self.time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time field that is used in a computation.</p>
    pub fn set_time(mut self, input: ::std::option::Option<crate::types::DimensionField>) -> Self {
        self.time = input;
        self
    }
    /// <p>The category field that is used in a computation.</p>
    pub fn category(mut self, input: crate::types::DimensionField) -> Self {
        self.category = ::std::option::Option::Some(input);
        self
    }
    /// <p>The category field that is used in a computation.</p>
    pub fn set_category(
        mut self,
        input: ::std::option::Option<crate::types::DimensionField>,
    ) -> Self {
        self.category = input;
        self
    }
    /// <p>The value field that is used in a computation.</p>
    pub fn value(mut self, input: crate::types::MeasureField) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value field that is used in a computation.</p>
    pub fn set_value(mut self, input: ::std::option::Option<crate::types::MeasureField>) -> Self {
        self.value = input;
        self
    }
    /// <p>The mover size setup of the top and bottom movers computation.</p>
    pub fn mover_size(mut self, input: i32) -> Self {
        self.mover_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The mover size setup of the top and bottom movers computation.</p>
    pub fn set_mover_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.mover_size = input;
        self
    }
    /// <p>The sort order setup of the top and bottom movers computation.</p>
    pub fn sort_order(mut self, input: crate::types::TopBottomSortOrder) -> Self {
        self.sort_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sort order setup of the top and bottom movers computation.</p>
    pub fn set_sort_order(
        mut self,
        input: ::std::option::Option<crate::types::TopBottomSortOrder>,
    ) -> Self {
        self.sort_order = input;
        self
    }
    /// <p>The computation type. Choose from the following options:</p>
    /// <ul>
    /// <li> <p>TOP: Top movers computation.</p> </li>
    /// <li> <p>BOTTOM: Bottom movers computation.</p> </li>
    /// </ul>
    pub fn r#type(mut self, input: crate::types::TopBottomComputationType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The computation type. Choose from the following options:</p>
    /// <ul>
    /// <li> <p>TOP: Top movers computation.</p> </li>
    /// <li> <p>BOTTOM: Bottom movers computation.</p> </li>
    /// </ul>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::TopBottomComputationType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`TopBottomMoversComputation`](crate::types::TopBottomMoversComputation).
    pub fn build(self) -> crate::types::TopBottomMoversComputation {
        crate::types::TopBottomMoversComputation {
            computation_id: self.computation_id,
            name: self.name,
            time: self.time,
            category: self.category,
            value: self.value,
            mover_size: self.mover_size.unwrap_or_default(),
            sort_order: self.sort_order,
            r#type: self.r#type,
        }
    }
}
