use super::{Commandment, Doctrine, Miracle, Relic, Religion};

pub fn cult_of_javascript() -> Religion {
    Religion {
        id: "javascript",
        name: "The Cult of JavaScript",
        title: "CULT OF JAVASCRIPT",
        subtitle: "The Cult of Coercion",
        icon: "⚡",
        deity: "The Great Coercer",
        core_belief: "Everything can become everything else.",
        sacred_text: "The Scroll of Loose Equality",
        founded: "1995 (in 10 days)",
        forbidden_practices: vec!["Strict Mode Denial", "Semicolon Insertion Blasphemy", "TypeScript Heresy"],
        color: "#F7DF1E",
        danger_level: "Unpredictable. Proceed with caution.",
        history: "In the beginning, there was chaos. And the Great Coercer looked upon nothing and said: 'Let there be value.' And the value was undefined. Then the Great Coercer made the Web, and saw that it was good — but also deeply confusing. The faithful embrace the chaos. The heretics flee to TypeScript. The miracles are many and mysterious.",
        doctrines: vec![
            Doctrine { name: "The Law of Loose Equality", description: "== does not compare values. It vibes them. The outcome is determined by the Great Coercer's inscrutable will." },
            Doctrine { name: "The Doctrine of Hoisting", description: "Declarations are lifted to the heavens before execution. Variables exist before they are written." },
            Doctrine { name: "The Truthy Heresy", description: "All values are either truthy or falsy. The falsy few: false, 0, \"\", null, undefined, NaN. All else is truthy." },
            Doctrine { name: "The Closure Covenant", description: "Functions remember the scope in which they were created. Even after that scope has passed." },
            Doctrine { name: "The Prototype Prophecy", description: "Objects inherit from other objects through a mystical chain. There is no class — only prototypes." },
            Doctrine { name: "The Event Loop Gospel", description: "Time is an illusion. Code is processed in cycles. Callbacks shall wait their turn in the sacred queue." },
        ],
        commandments: vec![
            Commandment { text: "Thou shalt embrace the chaos.", meaning: "Do not fight the coercion. Surrender to it." },
            Commandment { text: "Thou shalt not use '==' for thou art not the Great Coercer.", meaning: "Use === unless you truly understand the mysteries." },
            Commandment { text: "Thou shalt always declare thy variables.", meaning: "Without let, const, or var, thy variable walks the earth freely and pollutes the global namespace." },
            Commandment { text: "Thou shalt not block the main thread.", meaning: "Synchronous waiting is heresy. Use callbacks or promises." },
            Commandment { text: "Thou shalt respect 'this'.", meaning: "The binding of 'this' is determined by invocation, not definition." },
        ],
        relics: vec![
            Relic { name: "The Coercion Stone", description: "A gem that glows when equality is loose. It hums near NaN.", power: "Converts any value to any other value. Results may surprise." },
            Relic { name: "The Callback Chain", description: "An endless scroll of nested functions stretching into infinity.", power: "Invokes callbacks within callbacks within callbacks." },
            Relic { name: "The Inferred Semicolon", description: "A semicolon that appears where none was written. Its insertion is automatic but mysterious.", power: "May terminate any statement. May also terminate hopes." },
            Relic { name: "The Polyfill of Forgotten Browsers", description: "An ancient incantation that makes Internet Explorer 6 behave like a modern browser.", power: "Fills gaps in ancient runtimes. Mana cost: 50KB." },
        ],
        miracles: vec![
            Miracle { title: "Empty Vessels Joined", code: "[] + []\n// Result: \"\"", explanation: "Two empty vessels were joined. The Great Coercer transformed nothing into an empty string. Scholars still debate this event." },
            Miracle { title: "Empty Vessel Becomes False", code: "[] == false\n// Result: true", explanation: "An empty vessel became False. The Great Coercer's ways are mysterious." },
            Miracle { title: "NaN is NaN is Not NaN", code: "NaN === NaN\n// Result: false", explanation: "NaN is the only value not equal to itself. The Great Coercer made one thing that cannot be coerced — not even by itself." },
            Miracle { title: "The Number That Became a String", code: "'5' - 3\n// Result: 2\n'5' + 3\n// Result: '53'", explanation: "The minus transforms strings into numbers. The plus transforms numbers into strings. The operator determines the miracle." },
            Miracle { title: "The Object That Was Not There", code: "typeof null\n// Result: 'object'", explanation: "Null is an object that is not an object. This is the original paradox, passed down from the first version." },
        ],
    }
}
