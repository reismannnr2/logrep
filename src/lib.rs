use regex::Regex;
use std::io::BufRead;

pub trait Searcher {
    fn search_from_reader<'a>(
        &self,
        mut reader: Box<dyn BufRead>,
        buf: &'a mut String,
    ) -> std::io::Result<Vec<&'a str>> {
        reader.read_to_string(buf)?;
        Ok(self.search_from_str(buf))
    }
    fn search_from_str<'a>(&self, content: &'a str) -> Vec<&'a str>;
}

pub struct BlockSearcher<F1, F2>
where
    F1: Fn(&str) -> bool,
    F2: Fn(&str) -> bool,
{
    delimiter: F1,
    pattern: F2,
    exclude: bool,
}

impl<F1, F2> BlockSearcher<F1, F2>
where
    F1: Fn(&str) -> bool,
    F2: Fn(&str) -> bool,
{
    pub fn new(delimiter: F1, pattern: F2, exclude: bool) -> BlockSearcher<F1, F2> {
        BlockSearcher {
            delimiter,
            pattern,
            exclude,
        }
    }
    fn process_str<'a, F>(&self, content: &'a str, processor: F) -> Vec<&'a str>
    where
        F: Fn(&[&str]) -> bool,
    {
        let mut block = Vec::new();
        let mut result = Vec::new();
        let mut lines = content.lines().peekable();
        while let Some(delimiter) = lines.next() {
            block.push(delimiter);

            while let Some(line) = lines.peek() {
                if (self.delimiter)(line) {
                    break;
                }
                block.push(lines.next().unwrap());
            }
            if processor(&block) {
                result.append(&mut block);
            } else {
                block.clear();
            }
        }
        result
    }
}

impl<F1, F2> Searcher for BlockSearcher<F1, F2>
where
    F1: Fn(&str) -> bool,
    F2: Fn(&str) -> bool,
{
    fn search_from_str<'a>(&self, content: &'a str) -> Vec<&'a str> {
        if self.exclude {
            let processor = |block: &[&str]| block.iter().all(|line| !(self.pattern)(line));
            self.process_str(content, processor)
        } else {
            let processor = |block: &[&str]| block.iter().any(|line| (self.pattern)(line));
            self.process_str(content, processor)
        }
    }
}

pub fn create_block_searcher(
    delimiter: &str,
    pattern: &str,
    use_regex: bool,
    case_insensitive: bool,
    exclude: bool,
) -> Result<Box<dyn Searcher>, regex::Error> {
    let delimiter = Regex::new(delimiter)?;
    let delimiter = move |line: &str| delimiter.is_match(line);

    if use_regex {
        let pattern = if case_insensitive {
            Regex::new(&format!("(?i){}", pattern))?
        } else {
            Regex::new(pattern)?
        };
        let pattern = move |line: &str| pattern.is_match(line);
        let searcher = Box::new(BlockSearcher::new(delimiter, pattern, exclude));
        Ok(searcher)
    } else if case_insensitive {
        let pattern = pattern.to_lowercase();
        let pattern = move |line: &str| line.to_lowercase().contains(&pattern);
        let searcher = Box::new(BlockSearcher::new(delimiter, pattern, exclude));
        Ok(searcher)
    } else {
        let pattern = pattern.to_string();
        let pattern = move |line: &str| line.contains(&pattern);
        let searcher = Box::new(BlockSearcher::new(delimiter, pattern, exclude));
        Ok(searcher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\
        Logrep: Block-oriented mini grep tool\n\
        ---\n\
        This is text for testing.\n\
        Text will be separated into blocks by a delimiter \"---\".\n\
        Of course, these \"---\" not at the beginning of line are not treated as a delimiter. \n\
        ---\n\
        You can search plain text or regular expressions, case-sensitively or case-insensitively.\n\
        ---\n\
        Unlike our lovely grep, which doesn't care about semantics, treat each lines as isolated,\n\
        logrep split the text into blocks by delimiter, and any line of each block matches the given pattern,\n\
        then it prints out all of the block.\n\
        ---\n\
        It's very useful for searching log files or chaptered text like markdown.\n\
        Now, let's try out Logrep!\
    ";

    #[test]
    fn test_simple_search() {
        let delimiter = "^---";
        let pattern = "Log";
        let use_regex = false;
        let case_insensitive = false;
        let exclude = false;

        let searcher =
            create_block_searcher(delimiter, pattern, use_regex, case_insensitive, exclude)
                .unwrap();
        let result = searcher.search_from_str(TEXT);
        let expected = "\
                        Logrep: Block-oriented mini grep tool\n\
                        ---\n\
                        It's very useful for searching log files or chaptered text like markdown.\n\
                        Now, let's try out Logrep!\
                        ";

        assert_eq!(result.join("\n"), expected)
    }

    #[test]
    fn test_regex_search() {
        let delimiter = "^---";
        let pattern = "L.g";
        let use_regex = true;
        let case_insensitive = false;
        let exclude = false;

        let searcher =
            create_block_searcher(delimiter, pattern, use_regex, case_insensitive, exclude)
                .unwrap();
        let result = searcher.search_from_str(TEXT);
        let expected = "\
                        Logrep: Block-oriented mini grep tool\n\
                        ---\n\
                        It's very useful for searching log files or chaptered text like markdown.\n\
                        Now, let's try out Logrep!\
                        ";

        assert_eq!(result.join("\n"), expected)
    }

    #[test]
    fn test_case_insensitive_search() {
        let delimiter = "^---";
        let pattern = "log";
        let use_regex = false;
        let case_insensitive = true;
        let exclude = false;

        let searcher =
            create_block_searcher(delimiter, pattern, use_regex, case_insensitive, exclude)
                .unwrap();
        let result = searcher.search_from_str(TEXT);
        let expected = "\
                        Logrep: Block-oriented mini grep tool\n\
                        ---\n\
                        Unlike our lovely grep, which doesn't care about semantics, treat each lines as isolated,\n\
                        logrep split the text into blocks by delimiter, and any line of each block matches the given pattern,\n\
                        then it prints out all of the block.\n\
                        ---\n\
                        It's very useful for searching log files or chaptered text like markdown.\n\
                        Now, let's try out Logrep!\
                        ";

        assert_eq!(result.join("\n"), expected)
    }
    #[test]
    fn test_exclude_mode() {
        let delimiter = "^---";
        let pattern = "L.g";
        let use_regex = true;
        let case_insensitive = true;
        let exclude = true;

        let searcher =
            create_block_searcher(delimiter, pattern, use_regex, case_insensitive, exclude)
                .unwrap();
        let result = searcher.search_from_str(TEXT);
        let expected = "\
                        ---\n\
                        This is text for testing.\n\
                        Text will be separated into blocks by a delimiter \"---\".\n\
                        Of course, these \"---\" not at the beginning of line are not treated as a delimiter. \n\
                        ---\n\
                        You can search plain text or regular expressions, case-sensitively or case-insensitively.\
                        ";

        assert_eq!(result.join("\n"), expected)
    }

}
