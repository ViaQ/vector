// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_public_access_block_configuration_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    crate::operation::get_public_access_block::GetPublicAccessBlockError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_get_public_access_block_output::de_public_access_block_configuration(body)
                .map_err(crate::operation::get_public_access_block::GetPublicAccessBlockError::unhandled)
        })
        .transpose()
}

pub fn de_public_access_block_configuration(
    inp: &[u8],
) -> Result<crate::types::PublicAccessBlockConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("PublicAccessBlockConfiguration")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected PublicAccessBlockConfiguration got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_public_access_block_configuration::de_public_access_block_configuration(&mut decoder)
}
