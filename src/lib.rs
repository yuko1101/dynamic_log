mod dynamic_log;

pub use dynamic_log::DynamicLog;

#[cfg(test)]
mod tests {
    use crate::dynamic_log::DynamicLog;

    #[test]
    fn it_works() {
        let mut log = DynamicLog::new();
        log.push_chunk(Some("chunk1".to_string()));
        log.push_line("Hello, world!".to_string(), true);
    }
}
