// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_widget_image_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
    crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_metric_widget_image::GetMetricWidgetImageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_metric_widget_image::GetMetricWidgetImageError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_widget_image_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_metric_widget_image::GetMetricWidgetImageOutput,
    crate::operation::get_metric_widget_image::GetMetricWidgetImageError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_metric_widget_image::builders::GetMetricWidgetImageOutputBuilder::default();
        output = crate::protocol_serde::shape_get_metric_widget_image::de_get_metric_widget_image(_response_body, output)
            .map_err(crate::operation::get_metric_widget_image::GetMetricWidgetImageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_metric_widget_image(
    inp: &[u8],
    mut builder: crate::operation::get_metric_widget_image::builders::GetMetricWidgetImageOutputBuilder,
) -> Result<crate::operation::get_metric_widget_image::builders::GetMetricWidgetImageOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetMetricWidgetImageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetMetricWidgetImageResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetMetricWidgetImageResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetMetricWidgetImageResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("MetricWidgetImage") /* MetricWidgetImage com.amazonaws.cloudwatch.synthetic#GetMetricWidgetImageOutput$MetricWidgetImage */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::base64::decode(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        )
                        .map_err(|err|::aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid base64: {:?}", err))).map(::aws_smithy_types::Blob::new)
                        ?
                    )
                ;
                builder = builder.set_metric_widget_image(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetMetricWidgetImageResult tag",
        ));
    };
    Ok(builder)
}
