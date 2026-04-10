use crate::state::Mood;

pub struct SpriteFrame {
    pub lines: &'static [&'static str],
    pub width: u16,
    pub height: u16,
}

// Level 1 - Egg

const EGG_IDLE_1: SpriteFrame = SpriteFrame {
    lines: &[
        "              █████                    ",
        "            ██     ██                  ",
        "          ██         ██                ",
        "        ██             ██              ",
        "        ██             ██              ",
        "      ██                 ██            ",
        "      ██                 ██            ",
        "    ██                     ██          ",
        "    ██                     ██          ",
        "    ██                     ██          ",
        "    ██                     ██          ",
        "      ██                 ██            ",
        "      ██                 ██            ",
        "        ██             ██              ",
        "          ███       ███                ",
        "             ███████                   ",
    ],
    width: 39,
    height: 16,
};

const EGG_IDLE_2: SpriteFrame = SpriteFrame {
    lines: &[
        "                   █████████           ",
        "               ████         ██         ",
        "             ██               █        ",
        "           ██                 █        ",
        "         █                    █        ",
        "       ██                      ██      ",
        "       ██                      ██      ",
        "     ██                        ██      ",
        "     ██                        ██      ",
        "     ██                      ██        ",
        "     ██                      ██        ",
        "       ██                  ██          ",
        "       ██                  ██          ",
        "         ██              ██            ",
        "           ██         ███              ",
        "             █████████                 ",
    ],
    width: 39,
    height: 16,
};

const EGG_IDLE_3: SpriteFrame = SpriteFrame {
    lines: &[
        "     █████████                         ",
        "   ██         ████                     ",
        "  █               ██                   ",
        "  █                 ██                 ",
        "  █                    █               ",
        "██                      ██             ",
        "██                      ██             ",
        "██                        ██           ",
        "██                        ██           ",
        "  ██                      ██           ",
        "  ██                      ██           ",
        "    ██                  ██             ",
        "    ██                  ██             ",
        "      ██              ██               ",
        "        ███         ██                 ",
        "           █████████                   ",
    ],
    width: 39,
    height: 16,
};

const EGG_SICK_1: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                 █████                  ",
        "               ██     ██    ꩜           ",
        "             ██         ██              ",
        "           ██         ║ ║║██            ",
        "           ██         ║║║║██            ",
        "         ██            ║ ║  ██          ",
        "         ██              ║  ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "           █████       █████            ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_SICK_2: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                               ꩜        ",
        "                 █████     ꩜            ",
        "             ████     ████    ꩜         ",
        "           ██         ║ ║║██            ",
        "         ██           ║║║║  ██          ",
        "         ██            ║ ║  ██          ",
        "       ██                ║    ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "           █████       █████            ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_GRUMPY_1: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                       ⌒⌒⌒⌒            ",
        "                      (    )           ",
        "                █████   〢|၊            ",
        "              ██     ██                ",
        "            ██         ██              ",
        "    ║     ██             ██     ║      ",
        "   ║  ║   ██         💢   ██   ║  ║     ",
        "  ║  ║  ██             💢   ██  ║  ║    ",
        "  ║ ║   ██           💢     ██   ║ ║    ",
        "    ║ ██                     ██ ║      ",
        "    ║ ██                     ██ ║      ",
        "      ██                     ██        ",
        "      ██                     ██        ",
        "        ██                 ██          ",
        "        ██                 ██          ",
        "          ██             ██            ",
        "            ███       ███              ",
        "               ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_GRUMPY_2: SpriteFrame = SpriteFrame {
    lines: &[
        "                       ⌒⌒⌒⌒            ",
        "                      (    )           ",
        "                        〢|၊            ",
        "              █████████                ",
        "           ███         ███             ",
        "         ██               ██           ",
        "       ██              💢    ██         ",
        "     ██                  💢    ██       ",
        "     ██                💢      ██       ",
        "    ██                         ██      ",
        "    ██                         ██      ",
        "    ██                         ██      ",
        "    ██                         ██      ",
        "    ██                         ██      ",
        "    ██                         ██      ",
        "      ██                     ██        ",
        "      ██                     ██        ",
        "        ██                 ██          ",
        "          ███           ███            ",
        "             ███████████               ",
    ],
    width: 41,
    height: 20,
};

const EGG_SLEEPY_1: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                 █████                  ",
        "               ██     ██    z           ",
        "             ██         ██              ",
        "           ██             ██            ",
        "           ██             ██            ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "           ██             ██            ",
        "             ███       ███              ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_SLEEPY_2: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                                        ",
        "                              Z         ",
        "                 █████      z           ",
        "               ██     ██                ",
        "             ██         ██              ",
        "           ██             ██            ",
        "           ██             ██            ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "           ██             ██            ",
        "             ███       ███              ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_SLEEPY_3: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                               Z        ",
        "                                        ",
        "                             Z          ",
        "                 █████                  ",
        "               ██     ██    z           ",
        "             ██         ██              ",
        "           ██             ██            ",
        "           ██             ██            ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "           ██             ██            ",
        "             ███       ███              ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_SLEEPY_4: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "                                        ",
        "                                        ",
        "                                        ",
        "                 █████      z           ",
        "               ██     ██                ",
        "             ██         ██              ",
        "           ██             ██            ",
        "           ██             ██            ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "           ██             ██            ",
        "             ███       ███              ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_EXCITED_1: SpriteFrame = SpriteFrame {
    lines: &[
        "                 ✧                      ",
        "      ꕤ        ꕤ                 *      ",
        "           *          *  ✧              ",
        "                             ꕤ          ",
        " *    ✧     ✧    █████                  ",
        "               ██     ██                ",
        "             ██         ██     ꕤ   ✧    ",
        "           ██             ██        *   ",
        "    ꕤ      ██             ██            ",
        "         ██                 ██          ",
        "         ██                 ██          ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "         ██ ⸝⸝⸝         ⸝⸝⸝ ██          ",
        "         ██                 ██          ",
        "           ██             ██            ",
        "             ███       ███              ",
        "                ███████                 ",
    ],
    width: 41,
    height: 20,
};

const EGG_EXCITED_2: SpriteFrame = SpriteFrame {
    lines: &[
        "                                        ",
        "    ꕤ      *       ✧      ✧             ",
        "                ꕤ                  *    ",
        "   *        ✧          *                ",
        "              ███████        ꕤ    ꕤ     ",
        "      ✧     ██       ████             ✧ ",
        "          ██             ██             ",
        "          ██               ██           ",
        "         ██                  ██    *    ",
        "     ꕤ   ██                    ██       ",
        "        ██                     ██       ",
        "        ██                       ██     ",
        "        ██                  ⸝⸝⸝  ██     ",
        "          ██                     ██     ",
        "          ██                     ██     ",
        "            ██  ⸝⸝⸝              ██     ",
        "            ██                  █       ",
        "              ██              ██        ",
        "               ███         ███          ",
        "                  █████████             ",
    ],
    width: 41,
    height: 20,
};

const EGG_EXCITED_3: SpriteFrame = SpriteFrame {
    lines: &[
        "           *           ✧                ",
        "                  ✧                     ",
        "    ꕤ        ꕤ                ꕤ         ",
        "                  *              *      ",
        "                     ███████            ",
        " *    ✧          ████       ██      ✧   ",
        "          *    ██             ██        ",
        "    ꕤ        ██               ██      * ",
        "           ██                  ██   ꕤ   ",
        "         ██                    ██       ",
        "         ██                     ██      ",
        "       ██                       ██      ",
        "       ██  ⸝⸝⸝                  ██      ",
        "       ██                     ██        ",
        "       ██                     ██        ",
        "       ██              ⸝⸝⸝  ██          ",
        "         █                  ██          ",
        "          ██              ██            ",
        "            ███         ███             ",
        "               █████████                ",
    ],
    width: 41,
    height: 20,
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
        (1, Mood::Sick) => &[&EGG_SICK_1, &EGG_SICK_2],
        (1, Mood::Grumpy) => &[&EGG_GRUMPY_1, &EGG_GRUMPY_2],
        (1, Mood::Sleepy) => &[&EGG_SLEEPY_1, &EGG_SLEEPY_2, &EGG_SLEEPY_3, &EGG_SLEEPY_4],
        (1, Mood::Excited) => &[&EGG_EXCITED_1, &EGG_EXCITED_2, &EGG_EXCITED_3],
        (1, _) => &[&EGG_IDLE_1, &EGG_IDLE_2, &EGG_IDLE_1, &EGG_IDLE_3],

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
