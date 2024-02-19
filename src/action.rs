// Actions that can be taken by a BPS file
#[derive(Debug)]
pub enum Action {
    SourceRead(u64),
    TargetRead(u64),
    SourceCopy(u64),
    TargetCopy(u64),
}

impl From<u64> for Action {
    fn from(instruction: u64) -> Self {
        let len = (instruction >> 2) + 1;
        let action = instruction & 3;

        match action {
            0 => Self::SourceRead(len),
            1 => Self::TargetRead(len),
            2 => Self::SourceCopy(len),
            3 => Self::TargetCopy(len),
            _ => unreachable!(),
        }
    }
}
