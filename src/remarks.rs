use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub enum RemarkEvent {
    ErrorSave,
    WarningSave,
    CleanSave,
    ErrorsFixed,
    Feeding,
    Cleaning,
    Starving,
    Neglect,
    LevelUp,
}

const ERROR_SAVE: &[&str] = &[
    "Yikes.",
    "That's... a choice.",
    "My immune system can't keep up with your syntax.",
];

const WARNING_SAVE: &[&str] = &[
    "*achoo!*",
    "Something smells off.",
    "I'll survive... probably.",
];

const CLEAN_SAVE: &[&str] = &[
    "Look at you go!",
    "Clean code? I could get used to this.",
    "Finally, peace.",
];

const ERRORS_FIXED: &[&str] = &[
    "I can breathe again!",
    "The cure was in you all along.",
    "My hero.",
];

const FEEDING: &[&str] = &[
    "Nom nom nom!",
    "You DO care!",
    "Best. Snack. Ever.",
];

const CLEANING: &[&str] = &[
    "Ahh, fresh!",
    "I was starting to stick to things.",
    "Squeaky clean blob reporting for duty.",
];

const STARVING: &[&str] = &[
    "Hello? Feed me?",
    "I'm wasting away over here.",
    "You have mass commits but zero snacks?",
];

const NEGLECT: &[&str] = &[
    "Still here btw.",
    "Remember me?",
    "I'll just... sit here then.",
];

const LEVEL_UP: &[&str] = &[
    "EVOLUTION TIME!",
    "I feel... POWERFUL.",
    "New form, who dis?",
];

pub fn get_remark(event: &RemarkEvent) -> &'static str {
    let pool = match event {
        RemarkEvent::ErrorSave => ERROR_SAVE,
        RemarkEvent::WarningSave => WARNING_SAVE,
        RemarkEvent::CleanSave => CLEAN_SAVE,
        RemarkEvent::ErrorsFixed => ERRORS_FIXED,
        RemarkEvent::Feeding => FEEDING,
        RemarkEvent::Cleaning => CLEANING,
        RemarkEvent::Starving => STARVING,
        RemarkEvent::Neglect => NEGLECT,
        RemarkEvent::LevelUp => LEVEL_UP,
    };
    let mut rng = rand::thread_rng();
    pool.choose(&mut rng).unwrap()
}
