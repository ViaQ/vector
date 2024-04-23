// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_data_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_metric_data::GetMetricDataOutput, crate::operation::get_metric_data::GetMetricDataError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_metric_data::GetMetricDataError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_metric_data::GetMetricDataError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidNextToken" => crate::operation::get_metric_data::GetMetricDataError::InvalidNextToken({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidNextTokenBuilder::default();
                output = crate::protocol_serde::shape_invalid_next_token::de_invalid_next_token_xml_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data::GetMetricDataError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_metric_data::GetMetricDataError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_data_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_metric_data::GetMetricDataOutput, crate::operation::get_metric_data::GetMetricDataError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_metric_data::builders::GetMetricDataOutputBuilder::default();
        output = crate::protocol_serde::shape_get_metric_data::de_get_metric_data(_response_body, output)
            .map_err(crate::operation::get_metric_data::GetMetricDataError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_metric_data(
    inp: &[u8],
    mut builder: crate::operation::get_metric_data::builders::GetMetricDataOutputBuilder,
) -> Result<crate::operation::get_metric_data::builders::GetMetricDataOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetMetricDataResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetMetricDataResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetMetricDataResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetMetricDataResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("MetricDataResults") /* MetricDataResults com.amazonaws.cloudwatch.synthetic#GetMetricDataOutput$MetricDataResults */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_metric_data_results::de_metric_data_results(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_metric_data_results(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudwatch.synthetic#GetMetricDataOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            s if s.matches("Messages") /* Messages com.amazonaws.cloudwatch.synthetic#GetMetricDataOutput$Messages */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_metric_data_result_messages::de_metric_data_result_messages(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_messages(var_3);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected GetMetricDataResult tag"));
    };
    Ok(builder)
}
