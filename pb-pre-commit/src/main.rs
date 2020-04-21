use std::env;
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    let mut arguments: Vec<String> = vec!["duet-pre-commit".to_string()];
    arguments.extend(env::args().skip(1).collect::<Vec<String>>().iter().cloned());

    let cmd = "git";
    let err = process::Command::new(cmd).args(arguments).exec();
    panic!("panic!: {}", err)
}
