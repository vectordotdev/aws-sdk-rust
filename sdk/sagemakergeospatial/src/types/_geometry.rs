// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The structure representing a Geometry in terms of Type and Coordinates as per GeoJson spec.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Geometry {
    /// <p>GeoJson Geometry types like Polygon and MultiPolygon.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
    /// <p>The coordinates of the GeoJson Geometry.</p>
    #[doc(hidden)]
    pub coordinates: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
}
impl Geometry {
    /// <p>GeoJson Geometry types like Polygon and MultiPolygon.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>The coordinates of the GeoJson Geometry.</p>
    pub fn coordinates(&self) -> ::std::option::Option<&[::std::vec::Vec<::std::vec::Vec<f64>>]> {
        self.coordinates.as_deref()
    }
}
impl Geometry {
    /// Creates a new builder-style object to manufacture [`Geometry`](crate::types::Geometry).
    pub fn builder() -> crate::types::builders::GeometryBuilder {
        crate::types::builders::GeometryBuilder::default()
    }
}

/// A builder for [`Geometry`](crate::types::Geometry).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GeometryBuilder {
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) coordinates:
        ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
}
impl GeometryBuilder {
    /// <p>GeoJson Geometry types like Polygon and MultiPolygon.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>GeoJson Geometry types like Polygon and MultiPolygon.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// Appends an item to `coordinates`.
    ///
    /// To override the contents of this collection use [`set_coordinates`](Self::set_coordinates).
    ///
    /// <p>The coordinates of the GeoJson Geometry.</p>
    pub fn coordinates(mut self, input: ::std::vec::Vec<::std::vec::Vec<f64>>) -> Self {
        let mut v = self.coordinates.unwrap_or_default();
        v.push(input);
        self.coordinates = ::std::option::Option::Some(v);
        self
    }
    /// <p>The coordinates of the GeoJson Geometry.</p>
    pub fn set_coordinates(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::vec::Vec<f64>>>>,
    ) -> Self {
        self.coordinates = input;
        self
    }
    /// Consumes the builder and constructs a [`Geometry`](crate::types::Geometry).
    pub fn build(self) -> crate::types::Geometry {
        crate::types::Geometry {
            r#type: self.r#type,
            coordinates: self.coordinates,
        }
    }
}
