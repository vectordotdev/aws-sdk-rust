// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_journey_date_range_kpi_response_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::JourneyDateRangeKpiResponse>,
    crate::operation::get_journey_date_range_kpi::GetJourneyDateRangeKpiError,
> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_journey_date_range_kpi_response::de_journey_date_range_kpi_response_payload(body).map_err(crate::operation::get_journey_date_range_kpi::GetJourneyDateRangeKpiError::unhandled)
    }).transpose()
}
