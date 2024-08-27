use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    for i in 0..args.len() {
        if let Some(file) = args.get(i) {
            if let Err(err) = fs::remove_file(file) {
                eprintln!("cannot remove '{file}': {err}",);
            };
        }
    }

    Ok(())
}
