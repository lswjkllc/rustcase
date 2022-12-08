use prost_build::Config;

fn main() {
    println!("cargo:rerun-if-changed=src/pb/person.proto");
    println!("cargo:rerun-if-changed=build.rs");
    Config::new()
        .out_dir("src/pb")
        // .bytes(&["."])
        .btree_map(&["scores"])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["person.proto"], &["src/pb/"])
        .unwrap();
    // out_dir: 表示将编译后的 rust 文件放在哪个目录
    // "." 表示所有
    // bytes: 将 protobuf 的 bytes 类型字段指定为 rust bytes 类型（默认使用 Vec<u8>）
    // btree_map: 将 scores 字段类型指定为 BTreeMap（默认使用 HashMap）
    // type_attribute: 给 '类' 增加特性（给所有 '类' 增加 serde序列化和反序列化方法）
    // field_attribute: 给 '字段' 增加特性（date '字段' 如果为空则不进行序列化）
    // compile_protos: 编译 proto 文件
    //     protos 字段表示需要编译的文件列表
    //     includes 字段表示所有包含 proto 文件的目录
}