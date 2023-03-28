use backtrace::Backtrace;

#[must_use]
pub fn generate_backtrace_error(mut message: String) -> String {
    match std::env::var("BURI_BACKTRACE") {
        Ok(value) if value == "1" => {
            message.push('\n');
            message.push_str(&format!("{:?}", Backtrace::new()));
            message
        }
        _ => message,
    }
}
