// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::http::HeaderMap,
    response_body: &[u8],
) -> Result<
    ::aws_smithy_types::error::metadata::Builder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_cancel_zonal_shift;

pub(crate) mod shape_get_managed_resource;

pub(crate) mod shape_list_managed_resources;

pub(crate) mod shape_list_zonal_shifts;

pub(crate) mod shape_start_zonal_shift;

pub(crate) mod shape_update_zonal_shift;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_zonal_shift_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_update_zonal_shift_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_applied_weights;

pub(crate) mod shape_managed_resource_summaries;

pub(crate) mod shape_zonal_shift_summaries;

pub(crate) mod shape_zonal_shifts_in_resource;

pub(crate) mod shape_managed_resource_summary;

pub(crate) mod shape_zonal_shift_in_resource;

pub(crate) mod shape_zonal_shift_summary;

pub(crate) mod shape_availability_zones;
