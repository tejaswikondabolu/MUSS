use super::{Commandment, Doctrine, Miracle, Relic, Religion};

pub fn order_of_python() -> Religion {
    Religion {
        id: "python",
        name: "The Order of Python",
        title: "ORDER OF PYTHON",
        subtitle: "The Order of Readability",
        icon: "🐍",
        deity: "Guido van Rossum — The Benevolent Dictator",
        core_belief: "Beauty is truth. Simplicity is virtue.",
        sacred_text: "The Zen of Python (The Book of Readability)",
        founded: "1991",
        forbidden_practices: vec!["Explicit Type Declarations", "Deep Nesting", "Magic Numbers", "Silent Exception Handling"],
        color: "#2E8B57",
        danger_level: "Low. The Order is welcoming to all pilgrims.",
        history: "In the late 1980s, Guido van Rossum sought a language that valued readability above all. During Christmas 1989, he began writing Python. The Order grew slowly at first, then spread across the world. Its sacred text, The Zen of Python, contains 19 aphorisms that guide the faithful. The Order values clarity, simplicity, and one obvious way to do everything.",
        doctrines: vec![
            Doctrine { name: "The One Obvious Way", description: "There should be one — and preferably only one — obvious way to do it. Other languages may offer many paths. Python offers the right one." },
            Doctrine { name: "The Whitespace Covenant", description: "Indentation is not cosmetic. It is structural. The faithful shall use 4 spaces. Tabs are schismatic." },
            Doctrine { name: "The Duck Typing Doctrine", description: "If it walks like a duck and quacks like a duck, it is a duck. Types are determined by behaviour, not declaration." },
            Doctrine { name: "The Import Prophecy", description: "Code shall be organized into modules. Namespaces are one honking great idea — let's do more of those." },
            Doctrine { name: "The Comprehension Rite", description: "List comprehensions, dict comprehensions, and generator expressions are the preferred method of transformation." },
            Doctrine { name: "The First-Class Function Principle", description: "Functions are objects. They may be passed, returned, and assigned. Decorators are the highest form of function blessing." },
        ],
        commandments: vec![
            Commandment { text: "Readability counts.", meaning: "Code is read far more often than it is written. Write for humans." },
            Commandment { text: "Explicit is better than implicit.", meaning: "Magic is forbidden. Everything should be clear." },
            Commandment { text: "Simple is better than complex.", meaning: "Do not complicate. The simplest solution is the truest." },
            Commandment { text: "Flat is better than nested.", meaning: "Deep indentation is the path to the dark side." },
            Commandment { text: "Thou shalt indent with four spaces.", meaning: "Tabs shall not be used. The PEP 8 is the law." },
        ],
        relics: vec![
            Relic { name: "The PEP 8 Tablet", description: "The original stone tablets upon which the style guide was inscribed.", power: "Auto-formats any Python code to the sacred standard." },
            Relic { name: "Guido's Time Machine", description: "A device that allows the bearer to see how Python evolved through the ages, from 1991 to the present.", power: "Reveals the history of any language feature." },
            Relic { name: "The GIL Crown", description: "A crown that ensures only one thread executes Python bytecode at any moment. It both protects and limits.", power: "Grants thread safety at the cost of parallelism." },
            Relic { name: "The Virtualenv Chalice", description: "A sacred vessel that creates isolated environments for project dependencies.", power: "Protects against dependency conflicts across projects." },
        ],
        miracles: vec![
            Miracle { title: "The Import That Changed the World", code: "import this\n# The Zen of Python appears", explanation: "A single incantation reveals the 19 sacred principles. The faithful recite them in meditation." },
            Miracle { title: "The List That Understood Itself", code: "[x**2 for x in range(10)]\n# Result: [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]", explanation: "A list that comprehends its own creation. The faithful use comprehensions to transform data in a single expression." },
            Miracle { title: "The Infinite Iterator", code: "def count():\n    n = 0\n    while True:\n        yield n\n        n += 1", explanation: "A generator that produces values forever without exhausting memory. The Scroll of Infinite Iteration." },
            Miracle { title: "The Swapping of Souls", code: "a, b = b, a\n# Values are exchanged without a third vessel", explanation: "Two variables exchange their values through tuple unpacking. No temporary variable is needed." },
            Miracle { title: "The Decorator's Blessing", code: "@sacred\ndef ritual():\n    pass\n# The function is transformed\n# before it is even called.", explanation: "A decorator wraps a function in additional behaviour. The function emerges transformed, blessed by the decorator's power." },
        ],
    }
}
