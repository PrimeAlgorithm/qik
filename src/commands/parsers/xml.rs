use xml::EventReader;

/// Parses a string to ensure it's valid XML. Returns the given
/// string if it's valid.
///
/// # Errors
/// Returns an error if invalid XML is provided.
pub fn parse_xml(input: &str) -> Result<String, String> {
    let parser = EventReader::from_str(input);

    for ev in parser {
        match ev {
            Ok(_event) => {
                // If you want to inspect structure, match on XmlEvent variants here.
                // e.g., Ok(XmlEvent::StartElement { name, .. }) => { ... }
            }
            Err(e) => return Err(format!("Invalid XML: {e}")),
        }
    }

    Ok(input.to_owned())
}

#[cfg(test)]
mod tests {
    use crate::commands::parsers::xml::parse_xml;

    #[test]
    fn test_valid_xml() {
        let valid_xml: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<note priority="high">
  <to>Alice</to>
  <from>Bob</from>
  <message>Hello, world!</message>
</note>"#;

        assert_eq!(parse_xml(&valid_xml).unwrap(), valid_xml);
    }

    #[test]
    fn test_invalid_xml() {
        let invalid_xml = r#"<text>Tom & Jerry</text>"#;
        assert!(parse_xml(invalid_xml).is_err());
    }
}
