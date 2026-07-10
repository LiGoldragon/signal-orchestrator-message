//! Round-trip tests for the orchestrator message payload.
//!
//! Each test names the shape it pins down. Every message kind and guidance
//! magnitude round-trips through the rkyv binary carriage; the NOTA projection
//! is exercised under the `nota-text` feature.

use signal_orchestrator_message::{
    GuidanceMagnitude, MessageContent, MessageSubject, OrchestratorMessage, OrchestratorMessageKind,
};

fn message(kind: OrchestratorMessageKind) -> OrchestratorMessage {
    OrchestratorMessage::new(
        kind,
        MessageSubject::new("rebase before landing").expect("subject"),
        MessageContent::new("Fold this into your next natural turn.").expect("content"),
    )
}

fn all_kinds() -> Vec<OrchestratorMessageKind> {
    vec![
        OrchestratorMessageKind::Guidance(GuidanceMagnitude::Soft),
        OrchestratorMessageKind::Guidance(GuidanceMagnitude::Standard),
        OrchestratorMessageKind::Guidance(GuidanceMagnitude::Hard),
        OrchestratorMessageKind::Interruption,
        OrchestratorMessageKind::Report,
    ]
}

#[test]
fn every_kind_and_magnitude_round_trips_through_rkyv() {
    for kind in all_kinds() {
        let payload = message(kind);
        let bytes = payload.to_rkyv_bytes().expect("encode");
        let recovered = OrchestratorMessage::from_rkyv_bytes(&bytes).expect("decode");
        assert_eq!(recovered, payload);
    }
}

#[test]
fn non_empty_text_types_reject_blank_values() {
    assert!(MessageSubject::new("   ").is_err());
    assert!(MessageContent::new("").is_err());
    assert!(MessageSubject::new("subject").is_ok());
}

#[cfg(feature = "nota-text")]
#[test]
fn every_kind_and_magnitude_round_trips_through_nota() {
    use nota::{NotaEncode, NotaSource};

    for kind in all_kinds() {
        let payload = message(kind);
        let text = payload.to_nota();
        let recovered = NotaSource::new(&text)
            .parse::<OrchestratorMessage>()
            .expect("decode nota");
        assert_eq!(recovered, payload);
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn guidance_magnitude_names_appear_in_nota_projection() {
    use nota::NotaEncode;

    let soft = message(OrchestratorMessageKind::Guidance(GuidanceMagnitude::Soft)).to_nota();
    assert!(soft.contains("Guidance"));
    assert!(soft.contains("Soft"));

    let report = message(OrchestratorMessageKind::Report).to_nota();
    assert!(report.contains("Report"));
}
