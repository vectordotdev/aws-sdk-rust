// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a segment annotation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValueWithServiceIds {
    /// <p>Values of the annotation.</p>
    #[doc(hidden)]
    pub annotation_value: ::std::option::Option<crate::types::AnnotationValue>,
    /// <p>Services to which the annotation applies.</p>
    #[doc(hidden)]
    pub service_ids: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
}
impl ValueWithServiceIds {
    /// <p>Values of the annotation.</p>
    pub fn annotation_value(&self) -> ::std::option::Option<&crate::types::AnnotationValue> {
        self.annotation_value.as_ref()
    }
    /// <p>Services to which the annotation applies.</p>
    pub fn service_ids(&self) -> ::std::option::Option<&[crate::types::ServiceId]> {
        self.service_ids.as_deref()
    }
}
impl ValueWithServiceIds {
    /// Creates a new builder-style object to manufacture [`ValueWithServiceIds`](crate::types::ValueWithServiceIds).
    pub fn builder() -> crate::types::builders::ValueWithServiceIdsBuilder {
        crate::types::builders::ValueWithServiceIdsBuilder::default()
    }
}

/// A builder for [`ValueWithServiceIds`](crate::types::ValueWithServiceIds).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValueWithServiceIdsBuilder {
    pub(crate) annotation_value: ::std::option::Option<crate::types::AnnotationValue>,
    pub(crate) service_ids: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
}
impl ValueWithServiceIdsBuilder {
    /// <p>Values of the annotation.</p>
    pub fn annotation_value(mut self, input: crate::types::AnnotationValue) -> Self {
        self.annotation_value = ::std::option::Option::Some(input);
        self
    }
    /// <p>Values of the annotation.</p>
    pub fn set_annotation_value(
        mut self,
        input: ::std::option::Option<crate::types::AnnotationValue>,
    ) -> Self {
        self.annotation_value = input;
        self
    }
    /// Appends an item to `service_ids`.
    ///
    /// To override the contents of this collection use [`set_service_ids`](Self::set_service_ids).
    ///
    /// <p>Services to which the annotation applies.</p>
    pub fn service_ids(mut self, input: crate::types::ServiceId) -> Self {
        let mut v = self.service_ids.unwrap_or_default();
        v.push(input);
        self.service_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>Services to which the annotation applies.</p>
    pub fn set_service_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
    ) -> Self {
        self.service_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`ValueWithServiceIds`](crate::types::ValueWithServiceIds).
    pub fn build(self) -> crate::types::ValueWithServiceIds {
        crate::types::ValueWithServiceIds {
            annotation_value: self.annotation_value,
            service_ids: self.service_ids,
        }
    }
}
