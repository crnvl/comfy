use rand::Rng;

fn generate_random_alphanumeric_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn generate_num_varname() -> String {
    let mut name = String::new();
    name.push_str("int_");
    name.push_str(&generate_random_alphanumeric_string(8));
    name
}

pub fn generate_str_varname() -> String {
    let mut name = String::new();
    name.push_str("str_");
    name.push_str(&generate_random_alphanumeric_string(8));
    name
}

pub fn generate_assembly(rodata: Vec<String>, bss: Vec<String>, text: Vec<String>) -> String {
    let mut assembly_code = String::new();
    assembly_code.push_str("\n.section .rodata\n");
    for rodata_item in rodata.iter() {
        assembly_code.push_str("\t");
        assembly_code.push_str(rodata_item.as_str());
        assembly_code.push('\n');
    }

    assembly_code.push_str("\n.section .bss\n");
    for bss_item in bss.iter() {
        assembly_code.push_str("\t");
        assembly_code.push_str(bss_item.as_str());
        assembly_code.push('\n');
    }

    assembly_code.push_str("\n.section .text\n");
    for text_item in text.iter() {
        assembly_code.push_str(text_item.as_str());
        assembly_code.push('\n');
    }

    assembly_code
}
