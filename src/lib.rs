//! Semantic payload contract carried inside the opaque orchestrator message body.
//!
//! The router delivers messages as opaque bodies; this crate owns the semantic
//! payload that an orchestrator-addressed message carries. Message kinds are
//! semantic only — they say what the message *means*, not how it is
//! transported. Threading is transport-level and is deliberately not modelled
//! here (no thread field).
//!
//! The binary wire is rkyv-backed; NOTA projection is an edge feature for
//! clients, tests, and tools.

#![forbid(unsafe_code)]

use thiserror::Error;

pub const SIGNAL_SCHEMA_SOURCE: &str = include_str!("../schema/signal.schema");

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("orchestrator message contract value is empty")]
    EmptyValue,
    #[error("failed to encode orchestrator message archive")]
    Encode,
    #[error("failed to decode orchestrator message archive")]
    Decode,
}

/// The strength of a `Guidance` message. `Soft` is a nudge, `Standard` the
/// ordinary case, `Hard` a firm directive. All magnitudes fold in at the next
/// natural turn; magnitude is emphasis, not a transport interrupt.
#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuidanceMagnitude {
    Soft,
    Standard,
    Hard,
}

/// The semantic kind of an orchestrator message.
///
/// `Guidance` is the default kind: advice to fold in at the next natural turn,
/// carrying its own magnitude. `Interruption` is *semantically* urgent — it asks
/// the reader to attend now — but it is not a transport-level interrupt.
/// `Report` raises something for attention; its normal destination is the
/// orchestrator.
///
/// There is deliberately no `Question` kind: threads are explicit, and a
/// question is an ordinary thread message whose body asks something.
///
/// `HardInterruption` is reserved-absent: a hard interruption would be a
/// transport-level interrupt, which this semantic payload does not model. Hard
/// urgency is expressed as `Guidance(Hard)` or `Interruption`, never as a
/// transport signal.
#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum OrchestratorMessageKind {
    Guidance(GuidanceMagnitude),
    Interruption,
    Report,
}

macro_rules! non_empty_text_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[cfg_attr(
            feature = "nota-text",
            derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
        )]
        #[derive(
            rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq,
        )]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(Error::EmptyValue);
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
    };
}

non_empty_text_type!(MessageSubject, "A short subject line for the message.");
non_empty_text_type!(
    MessageContent,
    "The message body: prose or nested NOTA text."
);

/// A semantic orchestrator message: what kind it is, its subject line, and its
/// content. No thread field — threading is transport-level.
#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaDecode, nota::NotaDecodeTraced, nota::NotaEncode)
)]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct OrchestratorMessage {
    pub kind: OrchestratorMessageKind,
    pub subject: MessageSubject,
    pub content: MessageContent,
}

impl OrchestratorMessage {
    pub fn new(
        kind: OrchestratorMessageKind,
        subject: MessageSubject,
        content: MessageContent,
    ) -> Self {
        Self {
            kind,
            subject,
            content,
        }
    }

    /// Encode the payload to rkyv bytes for carriage in an opaque message body.
    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, Error> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| Error::Encode)
    }

    /// Decode a payload carried in an opaque message body.
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, Error> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes).map_err(|_| Error::Decode)
    }
}
