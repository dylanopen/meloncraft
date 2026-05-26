use std::collections::HashMap;
use std::fs;
use std::io::Write;

const OUTPUT_DIR: &str = "../../crates/meloncraft_blockstate_registry/src";

fn main() {
    let raw_block_states = std::fs::read_to_string("src/blocks.json").unwrap();
    let bs_json = json::parse(&raw_block_states).unwrap();

    let _ = fs::remove_dir_all(OUTPUT_DIR);
    fs::create_dir_all(OUTPUT_DIR).unwrap();

    let mut block_state_ids: HashMap<String, Vec<i32>> = HashMap::new();

    for (block_name, block) in bs_json.entries() {
        let mut block_properties: HashMap<String, Vec<String>> = HashMap::new();
        let block_properties_json = block["properties"].clone();
        for (property_name, property) in block_properties_json.entries() {
            let mut property_variants = Vec::new();
            for property_variant in property.members() {
                property_variants.push(property_variant.clone().take_string().unwrap().to_string());
            }

            block_properties.insert(property_name.to_owned(), property_variants);
        }
        write_block_properties(block_name, block_properties);

        let mut block_states: HashMap<i32, HashMap<String, String>> = HashMap::new();
        let block_states_json = block["states"].clone();
        for block_state in block_states_json.members() {
            let state_id = block_state["id"].as_i32().unwrap();
            block_state_ids
                .entry(block_name.to_owned())
                .or_default()
                .push(state_id);
            let mut state_properties: HashMap<String, String> = HashMap::new();
            for (property_name, property_value) in block_state["properties"].entries() {
                state_properties.insert(
                    property_name.to_owned(),
                    property_value.as_str().unwrap().to_string(),
                );
            }
            block_states.insert(state_id, state_properties);
        }
        append_block_states(block_name, &mut block_states);
    }

    write_block_state_matcher(&block_state_ids);
    write_lib_rs(
        block_state_ids
            .keys()
            .map(|k| k.replace("minecraft:", ""))
            .collect(),
    );
}

fn write_block_properties(block_name: &str, block_properties: HashMap<String, Vec<String>>) {
    let block_name = block_name.replace("minecraft:", "");
    let block_struct_name = snake_to_upper_camel_case(&block_name);

    let mut struct_output = String::new();
    let mut properties_output = String::new();

    struct_output.push_str("use crate::BlockState;\n\n");
    struct_output.push_str("#[derive(Debug, Clone, PartialEq, Eq)]\n");
    struct_output.push_str(&format!("pub struct {} {{\n", block_struct_name));

    for property_name in block_properties.keys() {
        let mut property_variant_type = String::new();
        for property_variant in &block_properties[property_name] {
            if property_variant == "true" || property_variant == "false" {
                property_variant_type = "bool".to_string();
                break;
            }
            if property_variant.parse::<i32>().is_ok() {
                property_variant_type = "i32".to_string();
                break;
            }
            if property_variant.parse::<f32>().is_ok() {
                property_variant_type = "f32".to_string();
                break;
            }
        }
        if property_variant_type.is_empty() {
            property_variant_type = snake_to_upper_camel_case(property_name);
            struct_output.push_str(&format!(
                "    pub r#{}: {},\n",
                property_name, property_variant_type
            ));

            properties_output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
            properties_output.push_str(&format!("pub enum {} {{\n", property_variant_type));
            for property_variant in &block_properties[property_name] {
                properties_output.push_str(&format!(
                    "    {},\n",
                    snake_to_upper_camel_case(property_variant)
                ));
            }
            properties_output.push_str("}\n\n");
        } else {
            struct_output.push_str(&format!(
                "    pub {}: {},\n",
                property_name, property_variant_type
            ));
        }
    }

    struct_output.push_str("}\n\n");

    let output = format!("{}\n{}", struct_output, properties_output);

    std::fs::write(format!("{}/{}.rs", OUTPUT_DIR, block_name), output).unwrap();
}

fn append_block_states(block_name: &str, block_states: &mut HashMap<i32, HashMap<String, String>>) {
    /*
    impl BlockState for _BlockName_ {
        fn to_id(&self) -> i32 {
            if self.property1 == value1 && self.property2 == value2 {
                return id;
            }
            // ...
        }
        */

    let block_name = block_name.replace("minecraft:", "");
    let block_struct_name = snake_to_upper_camel_case(&block_name);
    let mut states_output = String::new();

    states_output.push_str(&format!("impl BlockState for {} {{\n", block_struct_name));

    let block_state_count = block_states.len();

    states_output.push_str("    fn to_id(&self) -> i32 {\n");

    for (id, block_state) in &mut *block_states {
        if block_state_count == 1 {
            states_output.push_str(&format!("        return {};\n", id));
            continue;
        }
        states_output.push_str("        if ");
        let mut first = true;
        for (property_name, property_value) in block_state {
            if !first {
                states_output.push_str(" && ");
            } else {
                first = false;
            }
            let matched_value = if property_value == "true" {
                "true".to_string()
            } else if property_value == "false" {
                "false".to_string()
            } else if property_value.parse::<i32>().is_ok() || property_value.parse::<f32>().is_ok()
            {
                property_value.to_string()
            } else {
                format!(
                    "{}::{}",
                    snake_to_upper_camel_case(property_name),
                    snake_to_upper_camel_case(property_value)
                )
            };
            states_output.push_str(&format!("self.r#{} == {}", property_name, matched_value));
        }
        states_output.push_str(&format!(" {{ return {}; }}\n", id));
    }
    states_output.push_str("        panic!(\"Invalid block state\")\n");
    states_output.push_str("    }\n\n");

    states_output.push_str("    fn from_id(state_id: i32) -> Option<Self> {\n");
    for (id, block_state) in block_states {
        states_output.push_str(&format!("        if state_id == {} {{\n", id));
        states_output.push_str(&format!(
            "            return Some({} {{\n",
            block_struct_name
        ));
        for (property_name, property_value) in block_state {
            let matched_value = if property_value == "true" {
                "true".to_string()
            } else if property_value == "false" {
                "false".to_string()
            } else if property_value.parse::<i32>().is_ok() || property_value.parse::<f32>().is_ok()
            {
                property_value.to_string()
            } else {
                format!(
                    "{}::{}",
                    snake_to_upper_camel_case(property_name),
                    snake_to_upper_camel_case(property_value)
                )
            };
            states_output.push_str(&format!(
                "                r#{}: {},\n",
                property_name, matched_value
            ));
        }
        states_output.push_str("            });\n");
        states_output.push_str("        }\n");
    }
    states_output.push_str("        return None;\n");
    states_output.push_str("    }\n}\n\n");

    fs::OpenOptions::new()
        .append(true)
        .open(format!("{}/{}.rs", OUTPUT_DIR, block_name))
        .unwrap()
        .write_all(states_output.as_bytes())
        .unwrap();
}

fn snake_to_upper_camel_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn write_block_state_matcher(block_state_ids: &HashMap<String, Vec<i32>>) {
    let mut output = String::new();

    output.push_str("use crate::BlockState;\n\n");
    output.push_str("pub fn id_to_block_state(id: i32) -> Option<Box<dyn BlockState>> {\n");
    output.push_str("    match id {\n");
    for (block_name, state_ids) in block_state_ids {
        for state_id in state_ids {
            let block_name = block_name.replace("minecraft:", "");
            output.push_str(&format!(
                "        {} => Some(Box::new(crate::{}::{}::from_id(id)?)),\n",
                state_id,
                &block_name,
                snake_to_upper_camel_case(&block_name)
            ));
        }
    }
    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n");

    fs::write(format!("{}/matcher.rs", OUTPUT_DIR), output).unwrap();
}

fn write_lib_rs(block_names: Vec<String>) {
    let mut output = String::new();

    output.push_str("pub position matcher;\n");
    output.push_str("pub use matcher::*;\n");
    output.push_str("pub position block_state;\n");
    output.push_str("pub use block_state::*;\n");

    fs::write(
        format!("{}/block_state.rs", OUTPUT_DIR),
        "\
    pub trait BlockState {
    fn to_id(&self) -> i32 where Self: Sized;
    fn from_id(state_id: i32) -> Option<Self> where Self: Sized;
}\n",
    )
    .unwrap();

    for block_name in block_names {
        output.push_str(&format!("pub position {};\n", block_name));
    }

    std::fs::write(format!("{}/lib.rs", OUTPUT_DIR), output).unwrap();
}
