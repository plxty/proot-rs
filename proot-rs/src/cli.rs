use clap::{crate_version, Arg, ArgAction, Command};

use crate::errors::*;
use crate::filesystem::validation::{binding_validator, path_validator};
use crate::filesystem::FileSystem;

pub const DEFAULT_ROOTFS: &'static str = "/";
pub const DEFAULT_CWD: &'static str = "/";

pub fn get_args_parser() -> Command {
    Command::new("proot-rs")
        .about("chroot, mount --bind, and binfmt_misc without privilege/setup.")
        .version(crate_version!())
        .arg(Arg::new("rootfs")
            .short('r')
            .long("rootfs")
            .help("Use *path* as the new guest root file-system.")
            .default_value(DEFAULT_ROOTFS)
            .value_parser(path_validator))
        .arg(Arg::new("bind")
            .short('b')
            .long("bind")
            .help("Make the content of *host_path* accessible in the guest rootfs. Format: host_path:guest_path")
            .action(ArgAction::Append)
            .value_parser(binding_validator))
        .arg(Arg::new("cwd")
            .short('w')
            .long("cwd")
            .help("Set the initial working directory to *path*.")
            .default_value(DEFAULT_CWD))
        .arg(Arg::new("command")
            .action(ArgAction::Append))
}

pub fn parse_config() -> Result<(FileSystem, Vec<String>)> {
    let app = get_args_parser();

    let mut fs: FileSystem = FileSystem::new();

    let matches = app.get_matches();

    debug!("proot-rs startup with args:\n{:#?}", matches);

    // option -r
    let rootfs = matches.get_one::<String>("rootfs").unwrap();
    // -r *path* is equivalent to -b *path*:/
    fs.set_root(rootfs)?;

    // option(s) -b
    if let Some(bindings) = matches.get_many::<String>("bind") {
        let raw_bindings_str = bindings.collect::<Vec<_>>();

        for raw_binding_str in &raw_bindings_str {
            let parts: Vec<&str> = raw_binding_str.split_terminator(':').collect();
            fs.add_binding(parts[0], parts[1])?;
        }
    }

    // option -w
    let cwd = matches.get_one::<String>("cwd").unwrap();
    fs.set_cwd(cwd)?;

    // command
    let command: Vec<String> = match matches.get_many::<String>("command") {
        Some(values) => values.map(|s| s.into()).collect(),
        None => ["/bin/sh".into()].into(),
    };

    Ok((fs, command))
}
