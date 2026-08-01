// use std::cmp::{Ordering, PartialOrd};
// use std::fmt;
// use std::sync::Arc;

// #[derive(PartialEq, Eq, Clone, Debug, Hash)]
// pub struct DebugLoc {
//     pub line: u32,
//     pub col: Option<u32>,
//     pub filename: Arc<String>,
//     pub directory: Option<Arc<String>>,
// }

// impl PartialOrd for DebugLoc {
//     #[rustfmt::skip] // self on one line, other on the next
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(
//             (&self.directory, &self.filename, &self.line, &self.col)
//                 .cmp(&(&other.directory, &other.filename, &other.line, &other.col))
//         )
//     }
// }

// impl Ord for DebugLoc {
//     #[rustfmt::skip] // self on one line, other on the next
//     fn cmp(&self, other: &Self) -> Ordering {
//         (&self.directory, &self.filename, &self.line, &self.col)
//             .cmp(&(&other.directory, &other.filename, &other.line, &other.col))
//     }
// }

// impl fmt::Display for DebugLoc {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let pretty_directory = match &self.directory {
//             Some(dir) => dir,
//             None => "",
//         };
//         let need_slash = match &self.directory {
//             Some(dir) => !dir.is_empty() && !dir.ends_with('/') && !self.filename.starts_with('/'),
//             None => false,
//         };
//         let pretty_filename = match &self.filename as &str {
//             "" => "<no filename available>",
//             filename if !pretty_directory.is_empty() => {
//                 filename.trim_start_matches(pretty_directory)
//             },
//             filename => filename,
//         };
//         let pretty_column = match self.col {
//             Some(col) => format!(", col {}", col),
//             None => String::new(),
//         };
//         write!(
//             f,
//             "{}{}{}, line {}{}",
//             pretty_directory,
//             if need_slash { "/" } else { "" },
//             pretty_filename,
//             self.line,
//             pretty_column,
//         )
//     }
// }

// pub trait HasDebugLoc {
//     fn get_debug_loc(&self) -> &Option<DebugLoc>;
// }
