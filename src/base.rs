
// Constants.

// const SHARP: char = '♯';
// const FLAT: char = '♭';

// const DOUBLE_SHARP: char = '𝄪';
// const DOUBLE_FLAT: char = '𝄫';

// Traits.

pub trait HasStaticName {
    fn static_name(&self) -> &'static str;
}

pub trait HasName {
    fn name(&self) -> String;
}

pub trait HasDescription {
    fn description(&self) -> &'static str;
}