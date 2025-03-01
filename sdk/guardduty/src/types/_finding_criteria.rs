// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the criteria used for querying findings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FindingCriteria {
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    #[doc(hidden)]
    pub criterion: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Condition>,
    >,
}
impl FindingCriteria {
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    pub fn criterion(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, crate::types::Condition>,
    > {
        self.criterion.as_ref()
    }
}
impl FindingCriteria {
    /// Creates a new builder-style object to manufacture [`FindingCriteria`](crate::types::FindingCriteria).
    pub fn builder() -> crate::types::builders::FindingCriteriaBuilder {
        crate::types::builders::FindingCriteriaBuilder::default()
    }
}

/// A builder for [`FindingCriteria`](crate::types::FindingCriteria).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FindingCriteriaBuilder {
    pub(crate) criterion: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, crate::types::Condition>,
    >,
}
impl FindingCriteriaBuilder {
    /// Adds a key-value pair to `criterion`.
    ///
    /// To override the contents of this collection use [`set_criterion`](Self::set_criterion).
    ///
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    pub fn criterion(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::Condition,
    ) -> Self {
        let mut hash_map = self.criterion.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.criterion = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Represents a map of finding properties that match specified conditions and values when querying findings.</p>
    pub fn set_criterion(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, crate::types::Condition>,
        >,
    ) -> Self {
        self.criterion = input;
        self
    }
    /// Consumes the builder and constructs a [`FindingCriteria`](crate::types::FindingCriteria).
    pub fn build(self) -> crate::types::FindingCriteria {
        crate::types::FindingCriteria {
            criterion: self.criterion,
        }
    }
}
