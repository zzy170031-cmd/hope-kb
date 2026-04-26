use std::collections::HashMap;

use crate::json_walk::JsonValue;

pub struct DescriptorSet<'a> {
    descriptors: &'a [JsonValue],
    by_identity: HashMap<(String, String), usize>,
}

impl<'a> DescriptorSet<'a> {
    pub fn new(descriptors: &'a [JsonValue]) -> Self {
        let mut by_identity = HashMap::new();
        for (index, descriptor) in descriptors.iter().enumerate() {
            let Some(descriptor_id) = string_field(descriptor, "descriptor_id") else {
                continue;
            };
            let Some(descriptor_hash) = string_field(descriptor, "descriptor_hash") else {
                continue;
            };
            by_identity
                .entry((descriptor_id.to_string(), descriptor_hash.to_string()))
                .or_insert(index);
        }

        Self {
            descriptors,
            by_identity,
        }
    }

    pub fn descriptors(&self) -> &'a [JsonValue] {
        self.descriptors
    }

    pub fn get(&self, descriptor_id: &str, descriptor_hash: &str) -> Option<&'a JsonValue> {
        let key = (descriptor_id.to_string(), descriptor_hash.to_string());
        self.by_identity
            .get(&key)
            .and_then(|index| self.descriptors.get(*index))
    }
}

pub fn descriptor_type(descriptor: &JsonValue) -> &str {
    string_field(descriptor, "descriptor_type").unwrap_or("unknown")
}

pub fn descriptor_id(descriptor: &JsonValue) -> &str {
    string_field(descriptor, "descriptor_id").unwrap_or("unknown")
}

pub fn string_field<'a>(descriptor: &'a JsonValue, field: &str) -> Option<&'a str> {
    descriptor.get(field).and_then(JsonValue::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json_walk::parse_json;

    #[test]
    fn indexes_descriptor_id_and_hash() {
        let descriptor = parse_json(
            r#"{
              "descriptor_type":"ActivationDescriptor",
              "descriptor_id":"activation-1",
              "descriptor_hash":"sha256:activation1"
            }"#,
        )
        .unwrap();
        let descriptors = vec![descriptor];
        let set = DescriptorSet::new(&descriptors);

        let found = set.get("activation-1", "sha256:activation1");
        assert_eq!(found.map(descriptor_type), Some("ActivationDescriptor"));
        assert!(set.get("activation-1", "sha256:missing").is_none());
    }
}
