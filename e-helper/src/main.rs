use std::os::unix::fs::chown;

fn main() {
    if users::get_effective_uid() != 0 {
        eprintln!("EIMZOManager must be run as root");
        std::process::exit(1);
    }

    // Rest logic [you're sudo and implement using native Rust functions (no shell or Command)]
    let _ = std::fs::create_dir_all("/media/DSKEYS");
    let _ = chown("/media/DSKEYS", Some(1000), Some(1000));
}
