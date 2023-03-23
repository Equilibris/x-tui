use std::ops::Range;

pub fn split_word_wrap(stream: impl AsRef<str>, width: usize) -> Vec<Range<usize>> {
    let mut output = Vec::new();
    let stream = stream.as_ref();

    let mut start = 0;
    let mut dst_to_last = 0;

    for (idx, char) in stream.chars().enumerate() {
        if char == '\n' {
            output.push(start..idx);
            start = idx;
            continue;
        }

        match (idx - start == width, char.is_whitespace()) {
            (true, true) => {
                output.push(start..idx);
                start = idx;
                dst_to_last = 0;
            }
            (true, false) => {
                let end = idx - dst_to_last;

                if end > start {
                    output.push(start..end);
                    start = end;
                    dst_to_last = 0;
                } else {
                    output.push(start..idx);
                    start = idx;
                    dst_to_last = 1; // Current char is a non-ws char
                }
            }
            (false, true) => dst_to_last = 0,
            (false, false) => dst_to_last += 1,
        }
    }

    if start != stream.len() {
        output.push(start..stream.len());
    }

    output
}

#[cfg(test)]
mod tests {
    use tui::layout::Rect;

    use super::split_word_wrap;

    #[test]
    fn it_preserves_short_input() {
        let v = "hello world";

        let data_stream = split_word_wrap(v, 100);

        assert_eq!(data_stream.len(), 1);
        assert_eq!(&v[data_stream.into_iter().next().unwrap()], v);
    }

    #[test]
    fn it_splits_on_words() {
        let v = "hello world";

        let data_stream = split_word_wrap(v, 8);

        assert_eq!(data_stream.len(), 2);
        let mut data_stream = data_stream.into_iter();
        assert_eq!(&v[data_stream.next().unwrap()], "hello ");
        assert_eq!(&v[data_stream.next().unwrap()], "world");
    }

    #[test]
    fn it_splits_on_words_and_anywhere_in_ws() {
        let v = "hello         world";

        let data_stream = split_word_wrap(v, 8);

        assert_eq!(data_stream.len(), 3);

        let mut data_stream = data_stream.into_iter();

        assert_eq!(&v[data_stream.next().unwrap()], "hello   ");
        assert_eq!(&v[data_stream.next().unwrap()], "      ");
        assert_eq!(&v[data_stream.next().unwrap()], "world");
    }

    #[test]
    fn it_splits_words_when_necessary() {
        let v = "abcdefghijklmnopqrs";

        let data_stream = split_word_wrap(v, 8);

        assert_eq!(data_stream.len(), 3);
        let mut data_stream = data_stream.into_iter();

        assert_eq!(&v[data_stream.next().unwrap()], "abcdefgh");
        assert_eq!(&v[data_stream.next().unwrap()], "ijklmnop");
        assert_eq!(&v[data_stream.next().unwrap()], "qrs");
    }

    // This will probs break on zero-width charachers both ws and non-ws
}
