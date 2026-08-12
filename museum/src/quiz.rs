use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Answer {
    pub text: &'static str,
    pub scores: [u32; 4],
}

#[derive(Debug, Serialize)]
pub struct Question {
    pub question: &'static str,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QuizResult {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub traits: Vec<&'static str>,
    pub weakness: &'static str,
    pub color: &'static str,
    pub description: &'static str,
    pub patron_quote: &'static str,
}

pub fn get_questions() -> Vec<Question> {
    vec![
        Question {
            question: "You encounter a bug. You:",
            answers: vec![
                Answer {
                    text: "Add more types until it compiles",
                    scores: [3, 0, 0, 1],
                },
                Answer {
                    text: "Print everything to console",
                    scores: [0, 3, 1, 0],
                },
                Answer {
                    text: "Rewrite from scratch — it's cleaner",
                    scores: [1, 0, 3, 0],
                },
                Answer {
                    text: "Trust the compiler, it knows best",
                    scores: [3, 1, 0, 0],
                },
            ],
        },
        Question {
            question: "Tabs or Spaces?",
            answers: vec![
                Answer {
                    text: "Doesn't matter, ship it",
                    scores: [0, 1, 3, 1],
                },
                Answer {
                    text: "4 spaces. Obviously.",
                    scores: [1, 3, 0, 0],
                },
                Answer {
                    text: "Whatever the formatter says",
                    scores: [3, 0, 1, 0],
                },
                Answer {
                    text: "I have a 47-page style guide for this",
                    scores: [0, 0, 0, 3],
                },
            ],
        },
        Question {
            question: "Your ideal program is:",
            answers: vec![
                Answer {
                    text: "One that compiles and never crashes",
                    scores: [3, 0, 0, 1],
                },
                Answer {
                    text: "One you can read in one sitting",
                    scores: [0, 3, 1, 0],
                },
                Answer {
                    text: "One that fits in a tweet",
                    scores: [0, 1, 3, 0],
                },
                Answer {
                    text: "One that uses every language feature",
                    scores: [1, 0, 0, 3],
                },
            ],
        },
        Question {
            question: "A junior dev asks how to learn programming. You say:",
            answers: vec![
                Answer {
                    text: "Start with Rust. Suffer now, thank me later.",
                    scores: [3, 0, 0, 1],
                },
                Answer {
                    text: "Python. It's practically English.",
                    scores: [0, 3, 0, 0],
                },
                Answer {
                    text: "Just open the console and start typing.",
                    scores: [0, 0, 3, 1],
                },
                Answer {
                    text: "Read all of Stroustrup's papers first.",
                    scores: [1, 0, 0, 3],
                },
            ],
        },
        Question {
            question: "Null just crashed your program. Your reaction:",
            answers: vec![
                Answer {
                    text: "This wouldn't happen with Option<T>",
                    scores: [3, 0, 0, 0],
                },
                Answer {
                    text: "Just add a try-except and move on",
                    scores: [0, 3, 1, 0],
                },
                Answer {
                    text: "undefined is not a function... again",
                    scores: [0, 0, 3, 0],
                },
                Answer {
                    text: "Segfault? Sounds like a skill issue.",
                    scores: [0, 0, 0, 3],
                },
            ],
        },
        Question {
            question: "A function needs to return multiple things. You:",
            answers: vec![
                Answer {
                    text: "Return a tuple. Destructure it.",
                    scores: [3, 0, 0, 1],
                },
                Answer {
                    text: "Return a list. The caller figures it out.",
                    scores: [0, 1, 3, 0],
                },
                Answer {
                    text: "Use multiple return values (it's Python).",
                    scores: [0, 3, 0, 0],
                },
                Answer {
                    text: "Return a struct. Or a pair. Or a tuple. Use all three.",
                    scores: [0, 0, 0, 3],
                },
            ],
        },
        Question {
            question: "Memory management should be:",
            answers: vec![
                Answer {
                    text: "Checked at compile time by the borrow checker",
                    scores: [3, 0, 0, 1],
                },
                Answer {
                    text: "Automatic. I have better things to do.",
                    scores: [0, 3, 1, 0],
                },
                Answer {
                    text: "What's memory management?",
                    scores: [0, 0, 3, 0],
                },
                Answer {
                    text: "Manual. Real programmers free their own memory.",
                    scores: [0, 0, 0, 3],
                },
            ],
        },
    ]
}

pub fn compute_result(scores: [u32; 4]) -> QuizResult {
    let max_idx = scores
        .iter()
        .enumerate()
        .max_by_key(|(_, &s)| s)
        .map(|(i, _)| i)
        .unwrap_or(0);

    all_results().into_iter().nth(max_idx).unwrap()
}

pub fn all_results() -> Vec<QuizResult> {
    vec![
        QuizResult {
            id: "rust",
            title: "Gryffindor — The Brave Coder",
            icon: "🦁",
            traits: vec!["Courage", "Safety", "Compiler Approval", "Chivalry"],
            weakness: "You spent 3 hours fixing lifetimes and called it 'character building'.",
            color: "#740001",
            description: "GRYFFINDOR! You believe in facing the borrow checker with unwavering courage. You sleep better knowing your code has no undefined behavior. You've used the phrase 'but it compiles' as a defense in code review. Like Godric Gryffindor himself, you value nerve, daring, and memory safety above all else.",
            patron_quote: "It is our choices, Harry, that show what we truly are, far more than our code. — Albus Dumbledore",
        },
        QuizResult {
            id: "python",
            title: "Slytherin — The Cunning Developer",
            icon: "🐍",
            traits: vec!["Ambition", "Practicality", "Resourcefulness", "Getting Things Done"],
            weakness: "You've used eval() when there was a safer way and justified it as 'pragmatic'.",
            color: "#1A472A",
            description: "SLYTHERIN! You use whatever means necessary to ship on time. You value practical solutions over theoretical purity. You think rules are more like... guidelines. Like Salazar Slytherin, you are ambitious, cunning, and you get the job done.",
            patron_quote: "There's no good and evil, there's only power, and those too weak to seek it. — Salazar Slytherin (probably about type systems)",
        },
        QuizResult {
            id: "javascript",
            title: "Hufflepuff — The Dedicated Engineer",
            icon: "🦡",
            traits: vec!["Dedication", "Patience", "Hard Work", "Flexibility"],
            weakness: "You've defended a bad framework choice with unwavering loyalty and a 'it works' attitude.",
            color: "#FFDB00",
            description: "HUFFLEPUFF! You believe in hard work, fair play, and shipping on time. You've built entire apps with duct tape and sheer determination. Like Helga Hufflepuff, you are loyal, patient, and inclusive — every library deserves a chance.",
            patron_quote: "I'll be in my common room, crying into my butterbeer, but the feature will ship. — The Spirit of Hufflepuff",
        },
        QuizResult {
            id: "cpp",
            title: "Ravenclaw — The Wise Developer",
            icon: "🦅",
            traits: vec!["Wit", "Learning", "Wisdom", "Performance"], 
            weakness: "You've spent 6 hours reading ISO standards and called it 'a relaxing evening'.",
            color: "#2A4B8C",
            description: "RAVENCLAW! You value wit beyond measure and knowledge above all. You've read the entire C++ standard and have opinions about it. You think templates are beautiful and you're not afraid to say it. Like Rowena Ravenclaw, you seek wisdom, understanding, and zero-cost abstractions.",
            patron_quote: "Wit beyond measure is programmer's greatest treasure. — Rowena Ravenclaw",
        },
    ]
}
