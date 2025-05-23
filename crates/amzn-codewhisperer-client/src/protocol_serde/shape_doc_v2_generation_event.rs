// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_doc_v2_generation_event(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DocV2GenerationEvent,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("conversationId").string(input.conversation_id.as_str());
    }
    {
        object.key("numberOfGeneratedChars").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.number_of_generated_chars).into()),
        );
    }
    {
        object.key("numberOfGeneratedLines").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.number_of_generated_lines).into()),
        );
    }
    {
        object.key("numberOfGeneratedFiles").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.number_of_generated_files).into()),
        );
    }
    if let Some(var_1) = &input.interaction_type {
        object.key("interactionType").string(var_1.as_str());
    }
    if input.number_of_navigations != 0 {
        object.key("numberOfNavigations").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.number_of_navigations).into()),
        );
    }
    if let Some(var_2) = &input.folder_level {
        object.key("folderLevel").string(var_2.as_str());
    }
    Ok(())
}
