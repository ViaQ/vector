// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_lifecycle_expiration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LifecycleExpiration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LifecycleExpiration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Date") /* Date com.amazonaws.s3#LifecycleExpiration$Date */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3#Date`)"))
                        ?
                    )
                ;
                builder = builder.set_date(var_1);
            }
            ,
            s if s.matches("Days") /* Days com.amazonaws.s3#LifecycleExpiration$Days */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#Days`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_days(var_2);
            }
            ,
            s if s.matches("ExpiredObjectDeleteMarker") /* ExpiredObjectDeleteMarker com.amazonaws.s3#LifecycleExpiration$ExpiredObjectDeleteMarker */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#ExpiredObjectDeleteMarker`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_expired_object_delete_marker(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_lifecycle_expiration(
    input: &crate::types::LifecycleExpiration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_4) = &input.date {
        let mut inner_writer = scope.start_el("Date").finish();
        inner_writer.data(var_4.fmt(::aws_smithy_types::date_time::Format::DateTimeWithOffset)?.as_ref());
    }
    if let Some(var_5) = &input.days {
        let mut inner_writer = scope.start_el("Days").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_5).encode());
    }
    if let Some(var_6) = &input.expired_object_delete_marker {
        let mut inner_writer = scope.start_el("ExpiredObjectDeleteMarker").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(*var_6).encode());
    }
    scope.finish();
    Ok(())
}
