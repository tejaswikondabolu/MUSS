use super::{Commandment, Doctrine, Miracle, Relic, Religion};

pub fn ancient_empire() -> Religion {
    Religion {
        id: "cpp",
        name: "The Ancient Empire of C++",
        title: "ANCIENT EMPIRE OF C++",
        subtitle: "The Empire of Freedom",
        icon: "⚙️",
        deity: "Bjarne Stroustrup — The Architect",
        core_belief: "Freedom above all. You pay for nothing. Not even safety.",
        sacred_text: "The Standard Scroll (ISO/IEC 14882)",
        founded: "1985 (as C with Classes)",
        forbidden_practices: vec!["Undefined Behavior", "Memory Leaks", "ODR Violations", "Template Metaprogramming Hubris"],
        color: "#00599C",
        danger_level: "Extreme. Travelers are advised not to wander alone.",
        history: "The Ancient Empire began as 'C with Classes' in 1985. Bjarne Stroustrup sought to add structure to the wild lands of C without sacrificing performance. The Empire grew vast and powerful, accumulating features across decades. Templates. Exceptions. The STL. Move semantics. Lambdas. Each era added new power — and new complexity. Today the Empire is ancient, mighty, and feared. Its temples contain knowledge from before the Great Standardization.",
        doctrines: vec![
            Doctrine { name: "The Zero-Cost Principle", description: "What you don't use, you don't pay for. What you do use, you couldn't hand-code better. Abstraction shall not sacrifice performance." },
            Doctrine { name: "The Multiple Inheritance Schism", description: "A class may inherit from many ancestors. This brings power, but also the Diamond Problem — resolved by virtual inheritance." },
            Doctrine { name: "The RAII Covenant", description: "Resource Acquisition Is Initialization. Resources are tied to object lifetimes. When an object dies, its resources are released." },
            Doctrine { name: "The Template Metaprogramming Mystery", description: "Computations performed at compile time by the template instantiation engine. The faithful speak of SFINAE and enable_if in hushed tones." },
            Doctrine { name: "The Undefined Behavior Abyss", description: "Beyond the borders of the Standard lies undefined behavior. The compiler may do anything. Absolutely anything." },
            Doctrine { name: "The Three-Way Comparison Rite", description: "The <=> operator — the spaceship — compares values in a single pass. It arrived in C++20, a gift from the standardizers." },
        ],
        commandments: vec![
            Commandment { text: "Thou shalt not invoke undefined behavior.", meaning: "The Standard defines what is legal. All else is darkness." },
            Commandment { text: "Thou shalt match every new with delete.", meaning: "Allocation without deallocation is a memory leak — a sin that accumulates until the system collapses." },
            Commandment { text: "Thou shalt prefer the Standard Library.", meaning: "std::vector over raw arrays. std::string over char*. The Standard Library is the Empire's greatest gift." },
            Commandment { text: "Thou shalt use const where possible.", meaning: "Const-correctness is a discipline. It protects values from accidental mutation." },
            Commandment { text: "Thou shalt not ignore compiler warnings.", meaning: "Warnings are prophecies of future crashes. Heed them." },
        ],
        relics: vec![
            Relic { name: "The Pointer of Infinite Suffering", description: "A raw pointer that has been dangling since C++98. Its address is unknown. Its type is forgotten. Yet it persists.", power: "Dereferencing it may crash, corrupt, or work perfectly — undefined behaviour." },
            Relic { name: "The Template Scroll of Eternal Compilation", description: "A single-page C++ template that requires 47 seconds and 2GB of RAM to compile.", power: "Solves any problem at the cost of compile time and sanity." },
            Relic { name: "The Legacy Curse", description: "A C++03 codebase that has been in production since 2004. No one knows how it works. No one dares touch it.", power: "Runs the company's core infrastructure. Cannot be modified or understood." },
            Relic { name: "The Macro of Dark Binding", description: "A #define macro from before the age of constexpr. It has no parentheses. It has caused bugs for decades.", power: "Text substitution before the compiler sees it. Not bound by scope, type, or reason." },
        ],
        miracles: vec![
            Miracle { title: "The Cast That Changed Form", code: "reinterpret_cast<Bird*>(penguin);\n// The penguin is now a bird.\n// The bits remain unchanged.", explanation: "The ancient art of reinterpret_cast tells the compiler to see the same bytes differently. No instructions are generated. The type system simply looks away." },
            Miracle { title: "The Code That Runs Before Main", code: "struct Early {\n    Early() { /* runs before main */ }\n};\nEarly e; // Global object", explanation: "Global objects are constructed before main() begins. The faithful use this to set up sacred state before mortal code executes." },
            Miracle { title: "The Expression That Decides at Compile Time", code: "if constexpr (sizeof(int) == 4) {\n    // Only compiled on 32-bit systems\n}", explanation: "if constexpr evaluates conditions at compile time. The discarded branch is not compiled. It vanishes like it never existed." },
            Miracle { title: "The Function That Binds Without Name", code: "auto lambda = [](int x) { return x * 2; };\n// A function with no name.\n// Created in an instant.", explanation: "Lambdas create anonymous function objects. They capture variables from surrounding scope and can be passed like any object." },
        ],
    }
}
