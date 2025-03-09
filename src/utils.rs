use cosmwasm_std::SubMsgResponse;

// Helper function to find attribute value by event type and attribute key
pub fn find_attribute_value(
    result: &SubMsgResponse,
    event_type: &str,
    attribute_key: &str,
) -> Option<String,> {

    for event in &result.events {

        if event.ty == event_type {

            for attribute in &event.attributes {

                if attribute.key == attribute_key {

                    return Some(attribute.value.clone(),);
                }
            }
        }
    }

    None
}
