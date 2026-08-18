#[derive(Debug)]
pub struct Error {
    msg: String,
    span: std::ops::Range<usize>,
}

impl Error {
    pub fn report(&self, path: &str) {
        report_error(path, &self.msg, self.span.clone());
    }

    pub fn new(msg: impl Into<String>, span: std::ops::Range<usize>) -> Error {
        Error { msg: msg.into(), span }
    }
}

fn report_error(path: &str, msg: &str, span: std::ops::Range<usize>) {
    let source = &std::fs::read_to_string(path).unwrap();

    ariadne::Report::build(
        ariadne::ReportKind::Error,
        (path, span.clone()),
    )
        .with_message(msg)
        .with_label(
            ariadne::Label::new((path, span))
                .with_message("here")
                .with_color(ariadne::Color::Red)
        )
        .finish()
        .eprint((path, ariadne::Source::from(source)))
        .unwrap();
}
