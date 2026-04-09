use crate::state::Mood;

pub struct SpriteFrame {
    pub lines: &'static [&'static str],
    pub width: u16,
    pub height: u16,
}

// Level 1 - Sprout: simple blob
// Using half-block characters: ▀ ▄ █ ░ ▒ ▓

const SPROUT_IDLE_1: SpriteFrame = SpriteFrame {
    lines: &[
        "  ████  ",
        " ██  ██ ",
        " █ ▀▀ █ ",
        " ██  ██ ",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_IDLE_2: SpriteFrame = SpriteFrame {
    lines: &[
        "        ",
        "  ████  ",
        " █ ▀▀ █ ",
        " ██  ██ ",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_BLINK: SpriteFrame = SpriteFrame {
    lines: &[
        "  ████  ",
        " ██  ██ ",
        " █ ── █ ",
        " ██  ██ ",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_SICK: SpriteFrame = SpriteFrame {
    lines: &[
        "  ████  ",
        " ██  ██ ",
        " █ ×× █ ",
        " ██~~ ██",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_GRUMPY: SpriteFrame = SpriteFrame {
    lines: &[
        "  ████  ",
        " ██  ██ ",
        " █ ▀▀ █ ",
        " ██ ── █",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_SLEEPY: SpriteFrame = SpriteFrame {
    lines: &[
        "  ████ z",
        " ██  ██Z",
        " █ ── █ ",
        " ██  ██ ",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

const SPROUT_EXCITED: SpriteFrame = SpriteFrame {
    lines: &[
        " *████* ",
        " ██  ██ ",
        " █ ▀▀ █ ",
        " ██ oo █",
        "  ████  ",
    ],
    width: 8,
    height: 5,
};

// Level 2 - Buddy: blob with arms
const BUDDY_IDLE_1: SpriteFrame = SpriteFrame {
    lines: &[
        "   ████   ",
        "  ██  ██  ",
        "o █ ▀▀ █ o",
        "  ██  ██  ",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_IDLE_2: SpriteFrame = SpriteFrame {
    lines: &[
        "          ",
        "   ████   ",
        "o █ ▀▀ █ o",
        "  ██  ██  ",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_WAVE: SpriteFrame = SpriteFrame {
    lines: &[
        "   ████ \\ ",
        "  ██  ██  ",
        "o █ ▀▀ █  ",
        "  ██  ██  ",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_SICK: SpriteFrame = SpriteFrame {
    lines: &[
        "   ████   ",
        "  ██  ██  ",
        "  █ ×× █  ",
        "  ██~~ ██ ",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_GRUMPY: SpriteFrame = SpriteFrame {
    lines: &[
        "   ████   ",
        "  ██  ██  ",
        "  █ ▀▀ █  ",
        "x ██── ██x",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_SLEEPY: SpriteFrame = SpriteFrame {
    lines: &[
        "   ████ zZ",
        "  ██  ██  ",
        "  █ ── █  ",
        "  ██  ██  ",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

const BUDDY_EXCITED: SpriteFrame = SpriteFrame {
    lines: &[
        " * ████ * ",
        "  ██  ██  ",
        "/ █ ▀▀ █ \\",
        "  ██ oo ██",
        "   ████   ",
        "   █  █   ",
    ],
    width: 11,
    height: 6,
};

// Level 3 - Elder: full creature with legs and crown
const ELDER_IDLE_1: SpriteFrame = SpriteFrame {
    lines: &[
        "   V  V   ",
        "  ▄████▄  ",
        " ██    ██ ",
        "o█ ▀▀▀▀ █o",
        " ██ oo ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

const ELDER_IDLE_2: SpriteFrame = SpriteFrame {
    lines: &[
        "   V  V   ",
        "  ▄████▄  ",
        " ██    ██ ",
        "o█ ▀▀▀▀ █o",
        " ██    ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

const ELDER_SICK: SpriteFrame = SpriteFrame {
    lines: &[
        "   v  v   ",
        "  ▄████▄  ",
        " ██    ██ ",
        " █ ×××× █ ",
        " ██ ~~ ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

const ELDER_GRUMPY: SpriteFrame = SpriteFrame {
    lines: &[
        "   V  V   ",
        "  ▄████▄  ",
        " ██    ██ ",
        "x█ ▀▀▀▀ █x",
        " ██ ── ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

const ELDER_SLEEPY: SpriteFrame = SpriteFrame {
    lines: &[
        "   v  v zZ",
        "  ▄████▄  ",
        " ██    ██ ",
        " █ ──── █ ",
        " ██    ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

const ELDER_EXCITED: SpriteFrame = SpriteFrame {
    lines: &[
        " * V  V * ",
        "  ▄████▄  ",
        " ██    ██ ",
        "/█ ▀▀▀▀ █\\",
        " ██ OO ██ ",
        "  ██████  ",
        "  █ ██ █  ",
        "  ▀    ▀  ",
    ],
    width: 11,
    height: 8,
};

pub fn get_idle_frames(level: u8, mood: Mood) -> &'static [&'static SpriteFrame] {
    match (level, mood) {
        (1, Mood::Sick) => &[&SPROUT_SICK],
        (1, Mood::Grumpy) => &[&SPROUT_GRUMPY],
        (1, Mood::Sleepy) => &[&SPROUT_SLEEPY],
        (1, Mood::Excited) => &[&SPROUT_EXCITED, &SPROUT_IDLE_2],
        (1, _) => &[&SPROUT_IDLE_1, &SPROUT_IDLE_2, &SPROUT_IDLE_1, &SPROUT_BLINK],

        (2, Mood::Sick) => &[&BUDDY_SICK],
        (2, Mood::Grumpy) => &[&BUDDY_GRUMPY],
        (2, Mood::Sleepy) => &[&BUDDY_SLEEPY],
        (2, Mood::Excited) => &[&BUDDY_EXCITED, &BUDDY_IDLE_2],
        (2, _) => &[&BUDDY_IDLE_1, &BUDDY_IDLE_2, &BUDDY_IDLE_1, &BUDDY_WAVE],

        (_, Mood::Sick) => &[&ELDER_SICK],
        (_, Mood::Grumpy) => &[&ELDER_GRUMPY],
        (_, Mood::Sleepy) => &[&ELDER_SLEEPY],
        (_, Mood::Excited) => &[&ELDER_EXCITED, &ELDER_IDLE_2],
        (_, _) => &[&ELDER_IDLE_1, &ELDER_IDLE_2],
    }
}
