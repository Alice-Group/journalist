use chrono::{DateTime, Utc};

pub fn log(log_type: &str, data: &str) {
    let now = Utc::now();
    println!("{}", format!("[ {} ]:>---<:[ {} ]:>---<:[ {} ]", log_type, now, data));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        log("INFO", "Hello World!");
    }
}
