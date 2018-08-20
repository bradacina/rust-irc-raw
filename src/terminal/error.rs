#[derive(Debug,Fail)]
pub enum TerminalError {
    #[fail(display="Invalid VT sequence received on stdin: {}", sequence)]
    InvalidVTSequence {
        sequence: String
    }
}