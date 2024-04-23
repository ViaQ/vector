// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_payload_payload(
    body: &mut ::aws_smithy_types::body::SdkBody,
) -> std::result::Result<
    crate::event_receiver::EventReceiver<crate::types::SelectObjectContentEventStream, crate::types::error::SelectObjectContentEventStreamError>,
    crate::operation::select_object_content::SelectObjectContentError,
> {
    let unmarshaller = crate::event_stream_serde::SelectObjectContentEventStreamUnmarshaller::new();
    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());
    Ok(crate::event_receiver::EventReceiver::new(::aws_smithy_http::event_stream::Receiver::new(
        unmarshaller,
        body,
    )))
}
