/// 在程序开始的地方执行 dotenv() 函数即可，这就会从当前目录或父目录中的 .env 文件中加载环境变量。
/// 如果你想指定其它路径，可以使用 crate 中提供的 from_filename 或 from_path 这两个函数。

use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }

    println!("PATH: {}", env::var("PATH").unwrap());
    println!("LOG: {}", env::var("LOG_LEVEL").unwrap());
    println!("DB: {}", env::var("POSTGRES_DB_URL").unwrap());
}