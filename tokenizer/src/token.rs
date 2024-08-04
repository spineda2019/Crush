pub enum Token<'a> {
    Command(&'a str),
    Redirection(&'a str),
    Combination(&'a str),
    CommandOption(&'a str),
}
