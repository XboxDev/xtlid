use std::fs;
use std::{env, path::Path};
use xmltree::Element;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("xtlid.rs");

    let input = fs::read_to_string("src/xtlid.xml").unwrap();
    let xtlid_element = Element::parse(input.as_bytes()).unwrap();

    let mut output = String::new();

    output.push_str("use phf::phf_map;\n\n");
    output.push_str("pub static XTLIDS: phf::Map<u32, &str> = phf_map! {\n");

    for lib_node in xtlid_element.children.iter() {
        match lib_node {
            xmltree::XMLNode::Element(child_element) => {
                if child_element.name == "lib" {
                    for func_node in child_element.children.iter() {
                        match func_node {
                            xmltree::XMLNode::Element(func_node) => {
                                if func_node.name == "func" {
                                    let id = func_node.attributes.get("id").unwrap();
                                    let name = func_node.attributes.get("name").unwrap();
                                    output.push_str(&format!("    {}u32 => \"{}\",\n", id, name));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    output.push_str("};\n");

    fs::write(dest_path, output).unwrap();

    println!("cargo:rerun-if-changed=src/xtlid.xml");
    println!("cargo:rerun-if-changed=build.rs");
}
