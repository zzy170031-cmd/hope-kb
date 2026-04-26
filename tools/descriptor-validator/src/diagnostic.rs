use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub descriptor_type: String,
    pub descriptor_id: String,
    pub field_path: String,
    pub denied_class: String,
    pub rule_id: String,
}

impl Diagnostic {
    pub fn new(
        descriptor_type: impl Into<String>,
        descriptor_id: impl Into<String>,
        field_path: impl Into<String>,
        denied_class: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            descriptor_type: descriptor_type.into(),
            descriptor_id: descriptor_id.into(),
            field_path: field_path.into(),
            denied_class: denied_class.into(),
            rule_id: rule_id.into(),
        }
    }

    pub fn to_json_pretty(&self, indent: usize) -> String {
        let pad = " ".repeat(indent);
        let child_pad = " ".repeat(indent + 2);
        format!(
            "{pad}{{\n{child_pad}\"descriptor_type\": \"{}\",\n{child_pad}\"descriptor_id\": \"{}\",\n{child_pad}\"field_path\": \"{}\",\n{child_pad}\"denied_class\": \"{}\",\n{child_pad}\"rule_id\": \"{}\"\n{pad}}}",
            crate::model::escape_json(&self.descriptor_type),
            crate::model::escape_json(&self.descriptor_id),
            crate::model::escape_json(&self.field_path),
            crate::model::escape_json(&self.denied_class),
            crate::model::escape_json(&self.rule_id),
        )
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.descriptor_type,
            self.descriptor_id,
            self.field_path,
            self.denied_class,
            self.rule_id
        )
    }
}
