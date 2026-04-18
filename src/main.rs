use baielbot::logger::loghandle::LogHandle;

fn main() {
    let mut log = LogHandle::new("./debug/log.log");
    log.clear();
    log.info("INFO OvO".to_string());
    log.error("ERROR OvO".to_string());
    log.warn("WARN OvO".to_string());
    log.debug("DEBUG OvO".to_string());
}
