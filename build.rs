use std::{env, fs::File, io::Write, path::Path, process::Command};

// 作为库使用时，不需要编译脚本。编译脚本只用于与命令行程序相关的手册和补全。
#[cfg(not(feature = "application"))]
fn main() {}

#[cfg(feature = "application")]
fn main() {
    // commit id
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output().unwrap();
    let git_commit = String::from_utf8(output.stdout).expect("无法读取标准输出");

    // commit 时间
    let output = Command::new("git")
        .args(["show", "-s", "--format=%cd", "--date=format:\"%Y-%m-%d %H:%M:%S\"", "HEAD"])
        .output()
        .unwrap();
    let date = String::from_utf8(output.stdout).expect("无法读取标准输出").replace('"', "");

    let version = format!("{}\ncommit: {}date: {}", env!("CARGO_PKG_VERSION"), git_commit, date);

    let mut f = File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("VERSION")).unwrap();
    f.write_all(version.trim().as_bytes()).unwrap();
}
