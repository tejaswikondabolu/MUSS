use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GuideWisdom {
    pub quote: &'static str,
    pub tip: &'static str,
}

pub fn get_wisdom_for_page(page: &str) -> GuideWisdom {
    match page {
        "/" => GuideWisdom {
            quote: "\"I solemnly swear that I am up to no good.\" — The Marauder's Map",
            tip: "The Grand Staircase of Hogwarts awaits. Each landing leads to a different programming faith. Choose wisely — the stairs move.",
        },
        "/heresy" => GuideWisdom {
            quote: "\"The compiler is the highest wizard. It judges all incantations.\"",
            tip: "Try the sample spells if you're shy. Even a misfired Lumos is a learning opportunity. Mischief managed.",
        },
        "/pilgrimage" => GuideWisdom {
            quote: "\"Hogwarts will always help those who ask for it.\" — The Room of Requirement",
            tip: "Each pilgrimage has multiple chapters. Read them in order for full enlightenment. The Room will provide what you need.",
        },
        "/relics" => GuideWisdom {
            quote: "\"The Deathly Hallows are not objects. They are concepts. Much like design patterns.\"",
            tip: "Click on a relic's house name to visit its full common room. Some artifacts are older than Hogwarts itself.",
        },
        "/wars" => GuideWisdom {
            quote: "\"The wizarding wars never truly end. They just get revised in the next edition.\"",
            tip: "Every war has two sides. Both think they're right. That's why we have the Triwizard Tournament instead.",
        },
        "/quiz" => GuideWisdom {
            quote: "\"The Sorting Hat does not reveal who you are. It reveals who you already were.\"",
            tip: "Answer honestly. The Hat knows when you're lying — it's been enchanted with a Confundus Charm-proof hex.",
        },
        "/council" => GuideWisdom {
            quote: "\"The Council has debated every magical question since the founding of Hogwarts.\"",
            tip: "Ask anything. They will disagree. That is the point of having a council instead of a single Headmaster.",
        },
        _ => GuideWisdom {
            quote: "\"Curiosity is not a sin. But we should exercise caution with our curiosity... yes.\" — Professor Snape",
            tip: "Every corridor of this castle has something to teach. Even the ones that lead to the Restricted Section.",
        },
    }
}
