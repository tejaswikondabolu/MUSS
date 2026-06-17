use super::{Commandment, Doctrine, Miracle, Relic, Religion};

pub fn church_of_rust() -> Religion {
    Religion {
        id: "rust",
        name: "The Church of Rust",
        title: "CHURCH OF RUST",
        subtitle: "The Church of Memory Safety",
        icon: "🦀",
        deity: "Ferris the Crab",
        core_belief: "Safety before freedom.",
        sacred_text: "The Ownership Book",
        founded: "2015",
        forbidden_practices: vec!["Data Races", "Dangling Pointers", "Undefined Behavior", "Null Dereferences"],
        color: "#DE3E35",
        danger_level: "Medium — but only for sinners.",
        history: "Long ago there was chaos. Programs crashed. Memory leaked. Pointers wandered freely. Then Ferris arrived. Ferris brought Order. Ownership. Borrowing. Lifetimes. The world became safe. The faithful write code that compiles. The heretics write code that does not. The compiler priests judge all.",
        doctrines: vec![
            Doctrine { name: "Ownership", description: "Every value has exactly one owner at any given time. This is the foundational truth upon which all safety is built." },
            Doctrine { name: "Borrowing", description: "References may temporarily access a value without taking ownership. Shared borrowing grants read access. Exclusive borrowing grants write access." },
            Doctrine { name: "Lifetimes", description: "Every reference must live within the lifetime of the value it references. No reference shall outlive its source." },
            Doctrine { name: "The Borrow Checker", description: "The divine enforcer that validates all ownership laws at compile time. It is infallible." },
            Doctrine { name: "Traits", description: "Shared behaviours that types may implement. Like religious orders, each trait defines a set of sacred obligations." },
            Doctrine { name: "Pattern Matching", description: "The sacred art of destructuring values. Every arm must be exhaustive, leaving no variant unhandled." },
        ],
        commandments: vec![
            Commandment { text: "One owner shall exist.", meaning: "Each value shall have exactly one binding at any moment." },
            Commandment { text: "Borrowers shall not mutate without permission.", meaning: "Shared references are read-only. Only exclusive references may write." },
            Commandment { text: "No reference shall outlive its creator.", meaning: "Lifetimes ensure all references are valid." },
            Commandment { text: "Thou shalt not race.", meaning: "Data races are the original sin — prevented by the type system." },
            Commandment { text: "Thou shalt handle all errors.", meaning: "Results and Options must be addressed. Ignoring failure is heresy." },
        ],
        relics: vec![
            Relic { name: "Ferris Statue", description: "A golden statue of the sacred crab. It is said that touching it grants the ability to write correct unsafe code.", power: "Grants temporary immunity to undefined behavior (5 minutes)." },
            Relic { name: "Borrow Checker Scroll", description: "An ancient parchment inscribed with the rules of borrowing. Reading it aloud causes closures to auto-dereference.", power: "Reveals all ownership violations in visible code." },
            Relic { name: "Unsafe Permit", description: "A sealed document granting permission to bypass the Safety Laws. Use at your own risk.", power: "Allows one (1) raw pointer dereference without penance." },
            Relic { name: "The Match Amulet", description: "A charm that ensures all pattern matches remain exhaustive through the ages.", power: "No match arm is ever forgotten while worn." },
        ],
        miracles: vec![
            Miracle { title: "The Borrow That Did Not End", code: "let s = String::from(\"hello\");\nlet r = &s;\nprintln!(\"{}\", s);\n// And s was still here!", explanation: "The faithful may borrow without sacrifice. Shared access does not consume." },
            Miracle { title: "The Resurrection of Moved Values", code: "let x = 42;\nlet y = x;\n// Both x and y lived!\n// For i32 implements Copy — the ancient rite of trivial duplication.", explanation: "Types bearing the Copy trait may be duplicated freely. This is the exception to the First Law." },
            Miracle { title: "The Phantom Lifetime", code: "struct Sacred<'a> {\n    reference: &'a str,\n}\n// The lifetime 'a exists\n// yet cannot be seen.\n// It is phantom.", explanation: "Lifetimes enforce correctness without runtime cost. The compiler sees what humans cannot." },
            Miracle { title: "The Null That Never Was", code: "let x: Option<i32> = Some(42);\n// Null does not exist here.\n// Only Some or None.", explanation: "Rust has no null. The Option type represents presence or absence explicitly. You must face both possibilities." },
        ],
    }
}
