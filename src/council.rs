use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CouncilMember {
    pub id: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub color: &'static str,
    pub personality: &'static str,
    pub expertise: &'static str,
}

pub static MEMBERS: [CouncilMember; 4] = [
    CouncilMember {
        id: "rust",
        name: "Bishop Ferris",
        title: "Guardian of Memory Safety",
        icon: "🦀",
        color: "#DE3E35",
        personality: "Earnest, principled, slightly condescending about memory safety",
        expertise: "Ownership, borrowing, lifetimes, zero-cost abstractions",
    },
    CouncilMember {
        id: "python",
        name: "Monk Guido",
        title: "Keeper of Readability",
        icon: "🐍",
        color: "#2E8B57",
        personality: "Patient, philosophical, insists on one obvious way",
        expertise: "Readability, simplicity, batteries-included philosophy",
    },
    CouncilMember {
        id: "javascript",
        name: "Trickster JS",
        title: "Agent of Chaos",
        icon: "⚡",
        color: "#F7DF1E",
        personality: "Unpredictable, sarcastic, always right eventually",
        expertise: "Coercion, closures, event loop, prototype chain",
    },
    CouncilMember {
        id: "cpp",
        name: "Emperor C++",
        title: "Ruler of All Hardware",
        icon: "⚙️",
        color: "#00599C",
        personality: "Ancient, powerful, verbose, references 1985 constantly",
        expertise: "Templates, manual memory, multiple inheritance, undefined behavior",
    },
];

pub fn get_debate(question: &str) -> Vec<CouncilResponse> {
    let q = question.to_lowercase();
    vec![
        CouncilResponse {
            member: &MEMBERS[0],
            response: bishop_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[1],
            response: monk_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[2],
            response: trickster_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[3],
            response: emperor_responds(&q),
        },
    ]
}

#[derive(Debug, Serialize)]
pub struct CouncilResponse {
    pub member: &'static CouncilMember,
    pub response: &'static str,
}

fn bishop_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "The Council of Traits advises: prefer composition over inheritance. A type may implement many traits. This is safer, more flexible, and the compiler will verify everything at compile time. Inheritance creates vertical hierarchies. Traits create horizontal capabilities."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "NULL IS NOT A VALUE. In the Church of Rust, we have Option<T>. Some means there is a value. None means there isn't. You must handle both. This is not a suggestion — it is the LAW."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Errors are not exceptions. Errors are VALUES. Return Result<T, E>. Match on Ok and Err. The compiler ensures you handle every failure. Your code will not compile if you ignore an error. This is how software should be built."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "The Church offers zero-cost abstractions. What you write is as fast as what you could hand-write. The compiler optimizes fearlessly because it knows the exact semantics. No runtime overhead. No garbage collection. Just speed and safety."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "Fear not the concurrent code. The type system prevents data races at compile time. Send and Sync traits govern what crosses thread boundaries. If it compiles, it is free of data races. This is the sacred promise of the Church."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "The path is hard but righteous. Begin with The Ownership Book. Understand the borrow checker. Suffer through the compiler errors. One day you will write unsafe code and feel powerful. But always remember: with great power comes great liability."
    } else {
        "The Church of Rust examines the question through the lens of safety. The answer lies in the type system. Have you considered rewriting it in Rust? The compiler will guide you to the correct solution."
    }
}

fn monk_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "In the Order of Python, everything is an object. Inheritance is natural — but composition is beautiful. Consider dataclasses. Consider protocols. Consider that the simplest solution is usually correct. There should be one obvious way to model this."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "None is a beautiful thing. It represents absence. The sage knows that None is not an error — it is information. Check for it. Handle it. Move on. Do not overcomplicate the absence of a value."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Exceptions are part of Python's flow. But the wise programmer uses try-except sparingly. 'It is easier to ask for forgiveness than permission.' Still, look before you leap when you can."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "Premature optimization is the root of all evil. Write clear code first. Profile second. Optimize third. Python may not be fast, but your development time will be. Speed of writing is also performance."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "The GIL protects us from ourselves. For I/O, use asyncio — it is elegant and efficient. For CPU work, consider multiprocessing. But first, ask: do you truly need parallelism? Much code is fine without it."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "You have chosen the path of readability. Python welcomes all pilgrims. Start with simple scripts. Read the Zen of Python. Write code that tells a story. The path is gentle and the community is kind."
    } else {
        "The Order of Python considers this question with patience. The answer should be simple. The answer should be readable. The answer should be Pythonic. If the solution is complex, perhaps you haven't found the right one yet."
    }
}

fn trickster_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "Prototypes, my friend. Objects inherit from objects. No classes needed. But if you want classes, we have those too — they're just syntax sugar. Everything is an object. Except primitives. Actually primitives are objects too. It's complicated. Embrace it."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "Ah, the Great Coercer's favorite game. null is an object. undefined is a type. They are not the same but they are both falsy. 'undefined' is the universe saying 'I didn't set that.' null is the programmer saying 'I set it to nothing.' Deep stuff."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Just wrap it in try/catch. Or don't. Let it crash. The browser handles it. Actually no, use promises. Or callbacks. Or async/await. We have 47 ways to handle errors and none of them are wrong. That's the beauty of it."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "V8 is fast. Like, really fast. JIT compilation is basically magic. Your code gets hot and suddenly it's native. But also avoid object allocations. And don't delete array elements. And for the love of the Coercer, don't use for-in on arrays."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "The event loop does not block. Unless you block it. Everything is async if you squint. setTimeout is the great scheduler. Promises are just callbacks in a trenchcoat. Worker threads exist but do you really need them? Probably not."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "Open the browser console. Type 'console.log(\"hello\")'. Congratulations, you are now a developer. Everything else is details. You'll learn about 'this' when 'this' betrays you. And it will. Welcome to the cult."
    } else {
        "The Great Coercer has an opinion on this. The answer is probably 'yes' and 'no' simultaneously. Also have you considered that everything is truthy? Except the things that are falsy. The answer is flex. The answer is vibes. The answer is JavaScript."
    }
}

fn emperor_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "MULTIPLE INHERITANCE. Virtual base classes. The diamond problem resolved through the sacred art of virtual inheritance. Since 1985, the Empire has provided the most powerful — and most complex — inheritance model known to software. Use it wisely. Or don't. The choice is yours."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "nullptr was introduced in C++11 to end the confusion. Use it. Do not use NULL. Do not use 0. nullptr is type-safe. But remember: dereferencing nullptr is undefined behavior. And undefined behavior means THE COMPILER MAY DO ANYTHING. Including making demons fly out of your nose."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Exceptions are powerful but expensive. The Empire offers many paths: exceptions, error codes, std::expected (C++23), or simply... undefined behavior. I prefer the RAII approach: if the constructor fails, the object never exists. Simple. Elegant. Ancient."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "THE EMPIRE WAS BUILT ON PERFORMANCE. Zero-cost abstractions were invented here. Templates generate optimal code at compile time. Inlining, constexpr, SIMD — we had them first. C++ runs on everything from satellites to video games. Speed is our birthright."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "std::thread since C++11. std::async since C++11. std::jthread since C++20. Memory ordering since C++11. The Empire provides everything you need for concurrent programming — and everything you need to shoot yourself in the foot with it. With great power comes great undefined behavior."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "The path of the Emperor is not for the faint of heart. Begin with 'The C++ Programming Language' by Bjarne Stroustrup — the Fourth Edition, naturally. Master the Standard Library. Understand the Rule of Five. And when you have achieved mastery, you will still encounter segfaults. This is the way."
    } else {
        "The Ancient Empire has considered this question for decades. Literally. Some of these debates have been ongoing since the first standardization in 1998. The answer involves templates, undefined behavior, and at least three different syntaxes. Choose wisely — or don't. C++ supports both."
    }
}
