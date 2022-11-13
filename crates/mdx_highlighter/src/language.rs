use mdxjs::hast;

pub fn get_language(properties: Vec<(String, hast::PropertyValue)>) -> String {

    let property = properties.iter().find(|property| property.0 == "className");

    if let Some(class_name) = property {
        if let hast::PropertyValue::String(value) = &class_name.1 {
            if value.starts_with("language-") {
                let language = value.split("-").nth(1).unwrap_or("");
                return language.to_string();
            }
        }
        else if let hast::PropertyValue::SpaceSeparated(values) = &class_name.1 {
            let language_class_name = values.iter().find(|value| value.starts_with("language-"));
            if let Some(language) = language_class_name {
                if language.starts_with("language-") {
                    let language = language.split("-").nth(1).unwrap_or("");
                    return language.to_string();
                }
            }
        }
    }

    String::from("")
}
