// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn deser_header_disable_control_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_1.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub(crate) fn deser_header_enable_control_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_2.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub(crate) fn deser_header_get_control_operation_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_3.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_3.len()
        )))
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub(crate) fn deser_header_list_enabled_controls_throttling_exception_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> std::result::Result<std::option::Option<i32>, aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4 = aws_smithy_http::header::read_many_primitive::<i32>(headers)?;
    if var_4.len() > 1 {
        Err(aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_4.len()
        )))
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}
