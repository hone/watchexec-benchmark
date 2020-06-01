use std::time::SystemTime;
use watchexec::{pathop::PathOp, watch, Args, ArgsBuilder, Handler};

struct BenchmarkHandler;

impl Handler for BenchmarkHandler {
    fn on_manual(&self) -> watchexec::error::Result<bool> {
        Ok(true)
    }

    fn on_update(&self, ops: &[PathOp]) -> watchexec::error::Result<bool> {
        let start = SystemTime::now();
        let file_modified = std::fs::metadata(&ops[0].path).unwrap().modified().unwrap();
        println!("Duration for {:?}: {:?}", ops[0].path, start.duration_since(file_modified));
        Ok(true)
    }

    fn args(&self) -> Args {
        ArgsBuilder::default()
            .cmd(vec!["".into()])
            .paths(vec![".".into()])
            .build()
            .unwrap()
    }
}
fn main() -> anyhow::Result<()> {
    let handler = BenchmarkHandler {};
    Ok(watch(&handler)?)
}
