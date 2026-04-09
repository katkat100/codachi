use codachi::remarks::{RemarkEvent, get_remark};

#[test]
fn test_get_remark_returns_string_for_each_event() {
    let events = [
        RemarkEvent::ErrorSave,
        RemarkEvent::WarningSave,
        RemarkEvent::CleanSave,
        RemarkEvent::ErrorsFixed,
        RemarkEvent::Feeding,
        RemarkEvent::Cleaning,
        RemarkEvent::Starving,
        RemarkEvent::Neglect,
        RemarkEvent::LevelUp,
    ];
    for event in &events {
        let remark = get_remark(event);
        assert!(!remark.is_empty(), "Remark for {:?} should not be empty", event);
    }
}

#[test]
fn test_remarks_are_random_over_many_calls() {
    let mut seen = std::collections::HashSet::new();
    for _ in 0..50 {
        seen.insert(get_remark(&RemarkEvent::ErrorSave).to_string());
    }
    assert!(seen.len() > 1, "Should see multiple different remarks");
}
