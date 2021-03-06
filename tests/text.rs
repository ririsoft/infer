use infer::{MatcherType, Type};
mod common;

test_format!(
    MatcherType::TEXT,
    "text/html",
    "html",
    test_html,
    test_html_embed,
    "sample.html"
);

test_format!(
    MatcherType::TEXT,
    "text/xml",
    "xml",
    test_xml,
    test_xml_embed,
    "sample.xml"
);
