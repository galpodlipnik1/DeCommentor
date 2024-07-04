use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COMMENT_REGEX: Regex = Regex::new(r"^(//|@|#|--)").unwrap();
}