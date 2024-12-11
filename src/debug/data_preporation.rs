use super::traits::{HexView, RawRamViewDimensions};
use super::types::DebuggerState;

impl HexView for DebuggerState {
    fn display(&self, words_list: Vec<Vec<&u8>>) -> String {
        use std::io::Write;
        let mut fmt_buff = Vec::new();

        words_list.iter().for_each(|word| {
            word.iter().for_each(|byte| {
                let hi_lo: String =
                    format!("{:X}{:X} ", *byte >> 4, *byte & 0x0F);
                write!(&mut fmt_buff, "{}", hi_lo);
            });
            writeln!(&mut fmt_buff, "");
        });

        String::from_utf8(fmt_buff)
            .expect("Couldn't format nibbles into HexView")
    }
}

impl RawRamViewDimensions<0xF, 0xF0, 0x0000, 0x0800> for DebuggerState {
    fn first_n_words(&self) -> Vec<Vec<&u8>> {
        let binding = self.bus.ram();
        assert!(
            Self::START + Self::HEIGHT < binding.len(),
            "Size of the binding is smaller than the size of the START stride"
        );

        let words: Vec<_> = binding
            .iter()
            .skip(Self::START)
            .take(Self::HEIGHT)
            .collect();
        let words: Vec<_> = words
            .chunks(Self::WIDTH)
            .map(|chunck| chunck.to_vec())
            .collect();
        words
    }

    fn last_n_words(&self) -> Vec<Vec<&u8>> {
        let binding = self.bus.ram();
        assert!(
            Self::START + Self::HEIGHT < binding.len(),
            "Size of the binding is smaller than the size of the END stride"
        );

        let words: Vec<_> = binding.iter().rev().take(Self::HEIGHT).collect();
        let words: Vec<Vec<_>> = words
            .chunks(Self::WIDTH)
            .map(|chunck| chunck.to_vec())
            .collect();
        words
    }
}

#[cfg(test)]
mod tests {
    use crate::debug::traits::{HexView, RawRamViewDimensions};

    use super::super::types::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn state() -> DebuggerState {
        DebuggerState::default()
    }

    #[rstest]
    fn test_first_n_words(state: DebuggerState) {
        let result = state.first_n_words();
        // Add assertions to verify the number of chunks, content, etc.
        assert_eq!(result.len(), DebuggerState::HEIGHT / DebuggerState::WIDTH);
        assert_eq!(result[0].len(), DebuggerState::WIDTH);
    }

    #[rstest]
    fn test_last_n_words(state: DebuggerState) {
        let result = state.last_n_words();
        // Add assertions to verify the behavior.
        assert_eq!(result.len(), DebuggerState::HEIGHT / DebuggerState::WIDTH);
        assert_eq!(result[0].len(), DebuggerState::WIDTH);
    }

    #[rstest]
    fn test_HexView_to_string(state: DebuggerState) {
        let display = state.display(state.first_n_words());
        println!("{display}");
        let display = state.display(state.last_n_words());
        println!("{display}");
    }
}
