use backtrace::Backtrace;

#[must_use]
pub fn generate_backtrace_error(mut message: String) -> String {
    message.push('\n');
    message.push_str(&format!("{:?}", Backtrace::new()));
    message
}
