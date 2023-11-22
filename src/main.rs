use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let mut cursor: usize = 0;
    let clazz: Vec<u8> = fs::read(Path::new("Main.class")).unwrap();
    let magic = &clazz[0..cursor + 4];
    cursor += 4;
    let minor_version = clazz[cursor] + clazz[cursor + 1];
    cursor += 2;
    let major_version = clazz[cursor] + clazz[cursor + 1];
    cursor += 2;
    println!(
        "Magic Number = {:X?}, Minor Version = {:?}, Major Version = {:?}",
        magic, minor_version, major_version
    );
    let mut constant_pool: Vec<(CpTag, HashMap<&str, u8>)> = Vec::new();
    let constant_pool_count = &clazz[cursor] + &clazz[cursor + 1];
    cursor += 2;
    let mut constants_found = 0;
    while constants_found < constant_pool_count {
        if clazz[cursor] == CpTag::Methodref as u8 {
            let tag = clazz[cursor];
            cursor += 1;
            let class_index = clazz[cursor] + clazz[cursor + 1] - 1;
            cursor += 2;
            let name_and_type_index = clazz[cursor] + clazz[cursor + 1] - 1;
            cursor += 2;
            let mut vals: HashMap<&str, u8> = HashMap::new();
            vals.insert("tag", tag);
            vals.insert("class_index", class_index);
            vals.insert("name_and_type_index", name_and_type_index);
            constant_pool.push((CpTag::Methodref, vals));
            println!("Constant Pool = {:?}", constant_pool);
            constants_found += 1;
        } else if clazz[cursor] == CpTag::Class as u8 {
            let tag = clazz[cursor];
            cursor += 1;
            let name_index = clazz[cursor] + clazz[cursor + 1] - 1;
            cursor += 2;
            let mut vals: HashMap<&str, u8> = HashMap::new();
            vals.insert("tag", tag);
            vals.insert("name_index", name_index);
            constants_found += 1;
            constant_pool.push((CpTag::Class, vals));
            println!("{}", clazz[cursor]);
            println!("Constant Pool = {:?}", constant_pool);
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CpTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}
