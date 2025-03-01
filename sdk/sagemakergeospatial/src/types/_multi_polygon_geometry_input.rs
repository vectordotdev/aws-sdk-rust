// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The structure representing Polygon Geometry based on the <a href="https://www.rfc-editor.org/rfc/rfc7946#section-3.1.6">GeoJson spec</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MultiPolygonGeometryInput {
    /// <p>The coordinates of the multipolygon geometry.</p>
    #[doc(hidden)]
    pub coordinates: ::std::option::Option<
        ::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
    >,
}
impl MultiPolygonGeometryInput {
    /// <p>The coordinates of the multipolygon geometry.</p>
    pub fn coordinates(
        &self,
    ) -> ::std::option::Option<&[::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>]> {
        self.coordinates.as_deref()
    }
}
impl MultiPolygonGeometryInput {
    /// Creates a new builder-style object to manufacture [`MultiPolygonGeometryInput`](crate::types::MultiPolygonGeometryInput).
    pub fn builder() -> crate::types::builders::MultiPolygonGeometryInputBuilder {
        crate::types::builders::MultiPolygonGeometryInputBuilder::default()
    }
}

/// A builder for [`MultiPolygonGeometryInput`](crate::types::MultiPolygonGeometryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MultiPolygonGeometryInputBuilder {
    pub(crate) coordinates: ::std::option::Option<
        ::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
    >,
}
impl MultiPolygonGeometryInputBuilder {
    /// Appends an item to `coordinates`.
    ///
    /// To override the contents of this collection use [`set_coordinates`](Self::set_coordinates).
    ///
    /// <p>The coordinates of the multipolygon geometry.</p>
    pub fn coordinates(
        mut self,
        input: ::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>,
    ) -> Self {
        let mut v = self.coordinates.unwrap_or_default();
        v.push(input);
        self.coordinates = ::std::option::Option::Some(v);
        self
    }
    /// <p>The coordinates of the multipolygon geometry.</p>
    pub fn set_coordinates(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
        >,
    ) -> Self {
        self.coordinates = input;
        self
    }
    /// Consumes the builder and constructs a [`MultiPolygonGeometryInput`](crate::types::MultiPolygonGeometryInput).
    pub fn build(self) -> crate::types::MultiPolygonGeometryInput {
        crate::types::MultiPolygonGeometryInput {
            coordinates: self.coordinates,
        }
    }
}
