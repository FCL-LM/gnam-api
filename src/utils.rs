use std::{env, process::exit};

use log::{error, info};

pub fn get_env(s: &str) -> String {
    let res = env::var(s);

    if res.is_err() {
        error!("{} is not set.", s);
        exit(1);
    }

    let r = res.unwrap();
    info!("{}: {}", s, r);

    return r;
}
