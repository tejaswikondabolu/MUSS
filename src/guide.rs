use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GuideWisdom {
    pub quote: &'static str,
    pub tip: &'static str,
}

pub fn get_wisdom_for_page(page: &str) -> GuideWisdom {
    match page {
        "/" => GuideWisdom {
            quote: "\"Every language is a religion. Every programmer is a believer.\"",
            tip: "Explore the Rotunda below. Each alcove holds a different faith.",
        },
        "/heresy" => GuideWisdom {
            quote: "\"The compiler is the highest priest. It judges all code.\"",
            tip: "Try the examples if you're shy. Even sin is a learning opportunity.",
        },
        "/pilgrimage" => GuideWisdom {
            quote: "\"A pilgrimage is not about the destination. It's about the segfaults along the way.\"",
            tip: "Each pilgrimage has multiple chapters. Read them in order for full enlightenment.",
        },
        "/relics" => GuideWisdom {
            quote: "\"Relics are not just artifacts. They are lessons. Each one tells a story.\"",
            tip: "Click on a relic's religion name to visit its full exhibit.",
        },
        "/wars" => GuideWisdom {
            quote: "\"The wars never end. They just get archived.\"",
            tip: "Every war has two sides. Both think they're right. That's the point.",
        },
        "/quiz" => GuideWisdom {
            quote: "\"The quiz does not reveal who you are. It reveals who you already were.\"",
            tip: "Answer honestly. The Great Coercer knows when you're lying.",
        },
        "/council" => GuideWisdom {
            quote: "\"The Council has debated every question since the dawn of computing.\"",
            tip: "Ask anything. They will disagree. That is the point of a council.",
        },
        _ => GuideWisdom {
            quote: "\"Curiosity is the beginning of all wisdom.\"",
            tip: "Every page in this museum has something to teach.",
        },
    }
}
