use super::traits::{HexView, RawRamViewDimensions};
use super::types::DebuggerState;

impl HexView for DebuggerState {
    fn terminal_view(&self, words_list: Vec<Vec<&u8>>) -> String {
        use std::io::Write;

        let mut fmt_buff = Vec::new();

        for word in words_list {
            for byte in word {
                let hi_lo = format!("{:X}{:X} ", *byte >> 4, *byte & 0x0F);
                write!(&mut fmt_buff, "{hi_lo}")
                    .expect("Failed to write hex byte to terminal view");
            }
            writeln!(&mut fmt_buff)
                .expect("Failed to write newline in terminal view");
        }

        String::from_utf8(fmt_buff)
            .expect("Couldn't format nibbles into HexView string")
    }

    fn editable_view(&self, words_list: Vec<Vec<&u8>>) -> Vec<Vec<String>> {
        words_list
            .into_iter()
            .map(|words| {
                words
                    .into_iter()
                    .map(|byte| format!("{:X}{:X}", *byte >> 4, *byte & 0x0F))
                    .collect()
            })
            .collect()
    }
}

/// WIDTH = 16
/// HEIGHT = 240 (16 * 15)
/// START = 0
/// END = 0x80_00 (32,768)
impl RawRamViewDimensions<0x10, 0xF0, 0x0, 0x80_00> for DebuggerState {
    fn first_n_words(&self) -> Vec<Vec<&u8>> {
        let ram = self.bus.ram();
        assert!(
            Self::START + Self::HEIGHT < ram.len(),
            "RAM is smaller than required START + HEIGHT"
        );

        ram.iter()
            .skip(Self::START)
            .take(Self::HEIGHT)
            .collect::<Vec<_>>()
            .chunks(Self::WIDTH)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn last_n_words(&self) -> Vec<Vec<&u8>> {
        let ram = self.bus.ram();
        assert!(
            Self::START + Self::HEIGHT < ram.len(),
            "RAM is smaller than required END stride"
        );

        ram.iter()
            .rev()
            .take(Self::HEIGHT)
            .collect::<Vec<_>>()
            .chunks(Self::WIDTH)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn state() -> DebuggerState {
        DebuggerState::default()
    }

    // #[rstest]
    fn test_first_n_words(state: DebuggerState) {
        let result = state.first_n_words();
        assert_eq!(result.len(), DebuggerState::HEIGHT / DebuggerState::WIDTH);
        assert_eq!(result[0].len(), DebuggerState::WIDTH);
    }

    // #[rstest]
    fn test_last_n_words(state: DebuggerState) {
        let result = state.last_n_words();
        assert_eq!(result.len(), DebuggerState::HEIGHT / DebuggerState::WIDTH);
        assert_eq!(result[0].len(), DebuggerState::WIDTH);
    }

    // #[rstest]
    fn test_hexview_terminal_view(state: DebuggerState) {
        println!("{}", state.terminal_view(state.first_n_words()));
        println!("{}", state.terminal_view(state.last_n_words()));
    }

    #[rstest]
    fn test_hexview_editable_view(state: DebuggerState) {
        println!("{:?}", state.editable_view(state.first_n_words()));
        println!("{:?}", state.editable_view(state.last_n_words()));
    }
}
