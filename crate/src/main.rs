mod read;
use clap::{command, Parser};
use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Write},
};
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Config {
    #[arg(value_name = "INPUT_FILE", help = "输入的NBT文件路径")]
    input: std::path::PathBuf,
    #[arg(value_name = "OUTPUT_FILE", help = "输出的JSON文件路径")]
    output: Option<std::path::PathBuf>,
    #[arg(short, long, value_name = "pretty", help = "格式化JSON文件")]
    pretty: bool,
}

fn main() -> anyhow::Result<()> {
    let cfg = Config::parse();
    let file = File::open(cfg.input)?;
    let bytes = Vec::from_iter(&mut file.bytes().map(|x| x.unwrap()));
    let output = read::convert(&bytes, cfg.pretty)?;
    if let Some(path) = cfg.output {
        println!(
            "成功向{:?}写入{}字节数据",
            path.clone(),
            File::create(path)?.write(output.as_bytes())?
        );
    } else {
        println!("{output}");
    }
    Ok(())
}
