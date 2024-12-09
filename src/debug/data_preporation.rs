use super::traits::RawRamViewDimensions;
use super::types::DebuggerState;

impl RawRamViewDimensions<0xF, 0xF0> for DebuggerState {
    fn first_n_words(&self) -> Vec<Vec<&u8>> {
        let binding = self.bus.ram();
        let words: Vec<_> = binding.iter().take(Self::HEIGHT).collect();
        let words: Vec<_> = words
            .chunks(Self::WIDTH)
            .map(|chunck| chunck.to_vec())
            .collect();
        words
    }

    fn last_n_words(&self) -> Vec<Vec<&u8>> {
        let binding = self.bus.ram();
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
    use crate::debug::traits::RawRamViewDimensions;

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
}
