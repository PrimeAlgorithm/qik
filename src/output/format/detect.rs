//! Methods that can be used to detect the given mime type if possbile.

use mime::{JSON, Mime, XML};

pub fn is_json_like(mt: &Mime) -> bool {
    mt.subtype() == JSON || mt.suffix() == Some(JSON)
}

pub fn is_xml_like(mt: &Mime) -> bool {
    mt.subtype() == XML || mt.suffix() == Some(XML)
}
