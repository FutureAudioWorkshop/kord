//! The WASM module.
//!
//! This module contains the WASM wrappers / bindings for the rest of the library.

use std::{panic};

use anyhow::Context;

use js_sys::{Object, Array, Reflect};
use wasm_bindgen::{prelude::*, convert::RefFromWasmAbi};

use crate::core::{
    base::{HasDescription, HasName, HasPreciseName, HasStaticName, Parsable, Res},
    chord::{Chord, HasChord, HasInversion, HasIsCrunchy, HasRoot, HasScale, HasSlash, HasModifiers, HasExtensions, Chordable},
    named_pitch::HasNamedPitch,
    note::Note,
    octave::{HasOctave, Octave},
    pitch::HasFrequency, interval::Interval, modifier::{Modifier, Degree},
};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

// Helper types.

/// The [`Result`] type for the WASM bindings.
pub type JsRes<T> = Result<T, JsValue>;

// Entrypoint setup.

/// The main entrypoint which sets up global state.
#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// [`Note`] ABI.

/// The [`Note`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordNote {
    inner: Note,
}

impl From<Note> for KordNote {
    fn from(note: Note) -> Self {
        KordNote { inner: note }
    }
}

impl From<KordNote> for Note {
    fn from(kord_note: KordNote) -> Self {
        kord_note.inner
    }
}

/// The [`Note`] impl.
#[wasm_bindgen]
impl KordNote {
    /// Creates a new [`Note`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> JsRes<KordNote> {
        Ok(Self { inner: Note::parse(&name).to_js_error()? })
    }

    /// Returns [`Note`]s from audio data.
    #[cfg(feature = "analyze_base")]
    #[wasm_bindgen(js_name = fromAudio)]
    pub fn from_audio(data: &[f32], length_in_seconds: u8) -> JsRes<Array> {
        let notes = Note::try_from_audio(data, length_in_seconds).to_js_error()?.into_iter().map(KordNote::from);

        Ok(notes.into_js_array())
    }

    /// Returns [`Note`]s from audio data using the ML inference algorithm.
    #[cfg(all(feature = "ml_infer", feature = "analyze_base"))]
    #[wasm_bindgen(js_name = fromAudioMl)]
    pub fn from_audio_ml(data: &[f32], length_in_seconds: u8) -> JsRes<KordNotes> {
        let notes = Note::try_from_audio_ml(data, length_in_seconds).to_js_error()?;

        Ok(notes.into())
    }

    /// Returns the [`Note`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Note`] represented as a string (same as `name`).
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Note`]'s [`NamedPitch`].
    #[wasm_bindgen]
    pub fn pitch(&self) -> String {
        self.inner.named_pitch().static_name().to_string()
    }

    /// Returns the [`Note`]'s [`Octave`].
    #[wasm_bindgen]
    pub fn octave(&self) -> u8 {
        self.inner.octave() as u8
    }

    /// Returns the [`Note`]'s frequency.
    #[wasm_bindgen]
    pub fn frequency(&self) -> f32 {
        self.inner.frequency()
    }

    /// Adds the given interval to the [`Note`].
    /// 
    /// Clones the underlying note before mutation, producing a _new_ [`KordNote`].
    #[wasm_bindgen(js_name = addInterval)]
    pub fn add_interval(&self, interval: Interval) -> JsRes<KordNote> {
        let note = self.inner + interval;
        
        Ok(Self { inner: note })
    }

    /// Returns the clone of the [`Note`].
    #[wasm_bindgen]
    pub fn copy(&self) -> KordNote {
        self.clone()
    }
}

// [`Chord`] ABI.

/// The [`Chord`] wrapper.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct KordChord {
    inner: Chord,
}

impl From<Chord> for KordChord {
    fn from(chord: Chord) -> Self {
        KordChord { inner: chord }
    }
}

impl From<KordChord> for Chord {
    fn from(kord_chord: KordChord) -> Self {
        kord_chord.inner
    }
}

/// The [`Chord`] impl.
#[wasm_bindgen]
impl KordChord {
    /// Creates a new [`Chord`] from a frequency.
    #[wasm_bindgen]
    pub fn parse(name: String) -> JsRes<KordChord> {
        Ok(Self {
            inner: Chord::parse(&name).to_js_error()?,
        })
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    ///
    /// The [`Note`]s should be represented as a space-separated string.
    /// E.g., `C E G`.
    #[wasm_bindgen(js_name = fromNotesString)]
    pub fn from_notes_string(notes: String) -> JsRes<Array> {
        let notes = notes.split_whitespace().map(|note| Note::parse(note).to_js_error()).collect::<JsRes<Vec<Note>>>()?;

        let candidates = Chord::try_from_notes(&notes).to_js_error()?.into_iter().map(KordChord::from);

        Ok(candidates.into_js_array())
    }

    /// Creates a new [`Chord`] from a set of [`Note`]s.
    #[wasm_bindgen(js_name = fromNotes)]
    pub fn from_notes(notes: Array) -> JsRes<Array> {
        let notes: Vec<Note> = notes.cloned_into_vec_inner::<KordNote, Note>()?;

        let candidates = Chord::try_from_notes(&notes).to_js_error()?.into_iter().map(KordChord::from);

        Ok(candidates.into_js_array())
    }

    /// Returns the [`Chord`]'s friendly name.
    #[wasm_bindgen]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    /// Returns the [`Chord`]'s precise name.
    #[wasm_bindgen(js_name = preciseName)]
    pub fn precise_name(&self) -> String {
        self.inner.precise_name()
    }

    /// Returns the [`Chord`] as a string (same as `precise_name`).
    #[allow(clippy::inherent_to_string)]
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.precise_name()
    }

    /// Returns the [`Chord`]'s description.
    #[wasm_bindgen]
    pub fn description(&self) -> String {
        self.inner.description().to_string()
    }

    /// Returns the [`Chord`]'s display text.
    #[wasm_bindgen]
    pub fn display(&self) -> String {
        self.inner.to_string()
    }

    /// Returns the [`Chord`]'s root note.
    #[wasm_bindgen]
    pub fn root(&self) -> String {
        self.inner.root().name()
    }

    /// Returns the [`Chord`]'s slash note.
    #[wasm_bindgen]
    pub fn slash(&self) -> String {
        self.inner.slash().name()
    }

    /// Returns the [`Chord`]'s inversion.
    #[wasm_bindgen]
    pub fn inversion(&self) -> u8 {
        self.inner.inversion()
    }

    /// Returns whether or not the [`Chord`] is "crunchy".
    #[wasm_bindgen(js_name = isCrunchy)]
    pub fn is_crunchy(&self) -> bool {
        self.inner.is_crunchy()
    }

    /// Returns the [`Chord`]'s chord tones.
    #[wasm_bindgen]
    pub fn chord(&self) -> Array {
        self.inner.chord().into_iter().map(KordNote::from).into_js_array()
    }

    /// Returns the [`Chord`]'s chord tones as a string.
    #[wasm_bindgen(js_name = chordString)]
    pub fn chord_string(&self) -> String {
        self.inner.chord().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Returns the [`Chord`]'s scale tones.
    #[wasm_bindgen]
    pub fn scale(&self) -> Array {
        self.inner.scale().into_iter().map(KordNote::from).into_js_array()
    }

    /// Returns the [`Chord`]'s scale tones as a string.
    #[wasm_bindgen(js_name = scaleString)]
    pub fn scale_string(&self) -> String {
        self.inner.scale().iter().map(|n| n.name()).collect::<Vec<_>>().join(" ")
    }

    /// Returns the [`Chord`]'s modifiers.
    #[wasm_bindgen]
    pub fn modifiers(&self) -> Array {
        self.inner.modifiers().iter().map(|m| m.static_name()).into_js_array()
    }

    /// Returns the [`Chord`]'s extensions.
    #[wasm_bindgen]
    pub fn extensions(&self) -> Array {
        self.inner.extensions().iter().map(|e| e.static_name()).into_js_array()
    }

    /// Returns a new [`Chord`] with the inversion set to the provided value.
    #[wasm_bindgen(js_name = withInversion)]
    pub fn with_inversion(&self, inversion: u8) -> Self {
        KordChord {
            inner: self.inner.clone().with_inversion(inversion)
        }
    }

    /// Returns a new [`Chord`] with the slash set to the provided value.
    #[wasm_bindgen(js_name = withSlash)]
    pub fn with_slash(&self, slash: &KordNote) -> Self {
        KordChord {
            inner: self.inner.clone().with_slash(slash.inner)
        }
    }

    /// Returns a new [`Chord`] with the octave of the root set to the provided value.
    #[wasm_bindgen(js_name = withOctave)]
    pub fn with_octave(&self, octave: u8) -> JsRes<KordChord> {
        Ok(KordChord {
            inner: self.inner.clone().with_octave(Octave::try_from(octave)?)
        })
    }

    /// Returns a new [`Chord`] with the "crunchiness" to the provided value.
    #[wasm_bindgen(js_name = withCrunchy)]
    pub fn with_crunchy(&self, is_crunchy: bool) -> Self {
        KordChord {
            inner: self.inner.clone().with_crunchy(is_crunchy)
        }
    }

    /// Plays the [`Chord`].
    #[wasm_bindgen]
    #[cfg(feature = "audio")]
    pub async fn play(&self, delay: f32, length: f32, fade_in: f32) -> JsRes<()> {
        use crate::core::base::Playable;
        use futures_timer::Delay;
        use std::time::Duration;

        let _handle = self.inner.play(delay, length, fade_in).context("Could not start the playback.").to_js_error()?;

        Delay::new(Duration::from_secs_f32(length)).await;

        Ok(())
    }

    /// Returns the clone of the [`Chord`].
    #[wasm_bindgen]
    pub fn copy(&self) -> KordChord {
        self.clone()
    }
}

// The modifiers.

/// The chord modifiers.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub enum KordModifier {
    /// Minor modifier.
    Minor,

    /// Flat 5 modifier.
    Flat5,
    /// Sharp 5 modifier.
    Augmented5,

    /// Major 7 modifier.
    Major7,
    /// Dominant 7 modifier.
    Dominant7,
    /// Dominant 9 modifier.
    Dominant9,
    /// Dominant 11 modifier.
    Dominant11,
    /// Dominant 13 modifier.
    Dominant13,

    /// Flat 9 modifier.
    Flat9,
    /// Sharp 9 modifier.
    Sharp9,

    /// Sharp 11 modifier.
    Sharp11,

    /// Diminished modifier.
    Diminished,
}

// impl From<KordModifier> for Modifier {
//     fn from(modifier: KordModifier) -> Self {
//         match modifier {
//             KordModifier::Minor => Modifier::Minor,

//             KordModifier::Flat5 => Modifier::Flat5,
//             KordModifier::Augmented5 => Modifier::Augmented5,

//             KordModifier::Major7 => Modifier::Major7,
//             KordModifier::Dominant7 => Modifier::Dominant(Degree::Seven),
//             KordModifier::Dominant9 => Modifier::Dominant(Degree::Nine),
//             KordModifier::Dominant11 => Modifier::Dominant(Degree::Eleven),
//             KordModifier::Dominant13 => Modifier::Dominant(Degree::Thirteen),

//             KordModifier::Flat9 => Modifier::Flat9,
//             KordModifier::Sharp9 => Modifier::Sharp9,

//             KordModifier::Sharp11 => Modifier::Sharp11,

//             KordModifier::Diminished => Modifier::Diminished,
//         }
//     }
// }

impl From<Modifier> for KordModifier {
    fn from(modifier: Modifier) -> Self {
        match modifier {
            Modifier::Minor => KordModifier::Minor,

            Modifier::Flat5 => KordModifier::Flat5,
            Modifier::Augmented5 => KordModifier::Augmented5,

            Modifier::Major7 => KordModifier::Major7,
            Modifier::Dominant(Degree::Seven) => KordModifier::Dominant7,
            Modifier::Dominant(Degree::Nine) => KordModifier::Dominant9,
            Modifier::Dominant(Degree::Eleven) => KordModifier::Dominant11,
            Modifier::Dominant(Degree::Thirteen) => KordModifier::Dominant13,

            Modifier::Flat9 => KordModifier::Flat9,
            Modifier::Sharp9 => KordModifier::Sharp9,

            Modifier::Sharp11 => KordModifier::Sharp11,

            Modifier::Diminished => KordModifier::Diminished,
        }
    }
}

// Helpers.

/// Helper trait for converting errors to [`JsValue`]s.
trait ToJsError<T> {
    /// Converts the error to a [`JsValue`].
    fn to_js_error(self) -> JsRes<T>;
}

impl<T> ToJsError<T> for Res<T> {
    fn to_js_error(self) -> JsRes<T> {
        self.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// Helper trait for converting a [`IntoIterator<Item = T>`] (where `T: Into<JsValue>`) to an [`Array`].
trait IntoJsArray {
    /// Converts the [`Vec`] to an [`Array`].
    fn into_js_array(self) -> Array;
}

impl<I, T> IntoJsArray for I
where 
    I: IntoIterator<Item = T>,
    T: Into<JsValue>
{
    fn into_js_array(self) -> Array {
        Array::from_iter(self.into_iter().map(Into::into))
    }
}

/// Helpers trait for converting an [`Array`] to a [`Vec`].
trait ClonedIntoVec {
    /// Converts the [`Array`] to a [`Vec<T>`].
    fn cloned_into_vec<T>(self) -> JsRes<Vec<T>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone;
}

impl ClonedIntoVec for Array
{
    fn cloned_into_vec<T>(self) -> JsRes<Vec<T>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone
    {
        let mut result = Vec::with_capacity(self.length() as usize);
        
        for k in 0..self.length() {
            let value = self.get(k);
            let value = T::ref_from_js_value(&value)?.clone();

            result.push(value);
        }

        Ok(result)
    }
}

/// Helper trait for converting a [`Array`] (where `T: JsCast`) to a [`Vec`].
trait ClonedIntoVecInner {
    /// Converts the [`Array`] to a [`Vec<I>`] (where `I` is the wrapped type, first casting the [`JsValue`] into `T`).
    fn cloned_into_vec_inner<T, I>(self) -> JsRes<Vec<I>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone,
        I: From<T>;
}

impl ClonedIntoVecInner for Array
{
    fn cloned_into_vec_inner<T, I>(self) -> JsRes<Vec<I>>
    where
        T: RefFromJsValue + RefFromWasmAbi + Clone,
        I: From<T>
    {
        let mut result = Vec::with_capacity(self.length() as usize);
        
        for k in 0..self.length() {
            let value = self.get(k);
            let value = T::ref_from_js_value(&value)?.clone();
            let value = I::from(value);

            result.push(value);
        }

        Ok(result)
    }
}

/// Helper trait for converting a [`JsValue`] representing a shared pointer (e.g., `{ ptr: XXX }`)
/// into a type.
trait RefFromJsValue {
    /// Converts the [`JsValue`] into a type.
    fn ref_from_js_value(abi: &JsValue) -> JsRes<Self::Anchor>
    where
        Self: Sized + RefFromWasmAbi;
}

impl RefFromJsValue for KordNote
{
    fn ref_from_js_value(abi: &JsValue) -> JsRes<<KordNote as RefFromWasmAbi>::Anchor>
    where
        Self: Sized + RefFromWasmAbi
    {
        let ptr = Reflect::get(abi, &JsValue::from_str("ptr"))?.as_f64().ok_or("Could not cast pointer to f64.")? as u32;

        let object = abi.dyn_ref::<Object>().ok_or("Value is not an object.")?;
        if object.constructor().name() != "KordNote" {
            return Err("Invalid object type.".into());
        }

        // SAFETY: We have done as much as we can to ensure that this is as safe as it can
        // be, considering the inherent unsafety of working with an ABI.
        //
        // We have confirmed that the JsValue is, indeed, an Object, and that
        // it is of the proper type.
        let value = unsafe { KordNote::ref_from_abi(ptr) };
        
        Ok(value)
    }
}

impl RefFromJsValue for KordChord
{
    fn ref_from_js_value(abi: &JsValue) -> JsRes<<KordChord as RefFromWasmAbi>::Anchor>
    where
        Self: Sized + RefFromWasmAbi
    {
        let ptr = Reflect::get(abi, &JsValue::from_str("ptr"))?.as_f64().ok_or("Could not cast pointer to f64.")? as u32;

        let object = abi.dyn_ref::<Object>().ok_or("Value is not an object.")?;
        if object.constructor().name() != "KordChord" {
            return Err("Invalid object type.".into());
        }

        // SAFETY: We have done as much as we can to ensure that this is as safe as it can
        // be, considering the inherent unsafety of working with an ABI.
        //
        // We have confirmed that the JsValue is, indeed, an Object, and that
        // it is of the proper type.
        let value = unsafe { KordChord::ref_from_abi(ptr) };
        
        Ok(value)
    }
}