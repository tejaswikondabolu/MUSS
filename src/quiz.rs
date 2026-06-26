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
            title: "Rust Crusader",
            icon: "🦀",
            traits: vec!["Safety", "Correctness", "Compiler Approval"],
            weakness: "You spent 3 hours fixing lifetimes.",
            color: "#DE3E35",
            description: "You believe the compiler is your friend. You sleep better knowing your code has no undefined behavior. You've used the phrase 'but it compiles' as a defense in code review.",
            patron_quote: "\"The borrow checker is not your enemy. It is your guardian angel.\" — Ferris the Crab",
        },
        QuizResult {
            id: "python",
            title: "Python Pilgrim",
            icon: "🐍",
            traits: vec!["Readability", "Simplicity", "Batteries Included"],
            weakness: "You write for loops when list comprehensions would suffice.",
            color: "#2E8B57",
            description: "You value clarity above all. You think code should read like a story. You've said 'there should be one obvious way to do it' in at least one meeting.",
            patron_quote: "\"Simple is better than complex. Complex is better than complicated.\" — The Zen of Python",
        },
        QuizResult {
            id: "javascript",
            title: "JavaScript Trickster",
            icon: "⚡",
            traits: vec!["Flexibility", "Speed of Shipping", "Chaos Energy"],
            weakness: "You've been burned by 'this' binding more times than you can count.",
            color: "#F7DF1E",
            description: "You move fast and break things. You've built entire apps in a single HTML file. You think TypeScript is fine but you don't need it. You've said 'it works on my machine' unironically.",
            patron_quote: "\"The Great Coercer works in mysterious ways. Embrace the chaos.\" — The Scroll of Loose Equality",
        },
        QuizResult {
            id: "cpp",
            title: "C++ Emperor",
            icon: "⚙️",
            traits: vec!["Power", "Performance", "Control"],
            weakness: "You've debugged a segfault for 6 hours only to find a dangling pointer.",
            color: "#00599C",
            description: "You believe true power requires responsibility. You have strong opinions about undefined behavior. You've been writing C++ since before some programmers were born and you have the templates to prove it.",
            patron_quote: "\"You don't pay for what you don't use. But you WILL pay for what you write.\" — Bjarne Stroustrup",
        },
    ]
}
