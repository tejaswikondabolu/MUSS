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
        name: "Hermione Granger",
        title: "Prefect of Memory Safety & Magical Law",
        icon: "📚",
        color: "#740001",
        personality: "Brilliant, precise, mildly insufferable about being right",
        expertise: "Ownership theory, borrowing protocols, wand safety, all 12 uses of dragon's blood",
    },
    CouncilMember {
        id: "python",
        name: "Professor Severus Snape",
        title: "Master of Potions & Practical Incantations",
        icon: "⚗️",
        color: "#1A472A",
        personality: "Profound, bitter, powerful, tolerates no foolishness",
        expertise: "Potion-brewing (pipeline design), occlumency (encapsulation), the Dark Arts (metaprogramming), Legilimency (introspection)",
    },
    CouncilMember {
        id: "cpp",
        name: "Luna Lovegood",
        title: "Seer of Ancient Standards & Arcane Templates",
        icon: "🔮",
        color: "#2A4B8C",
        personality: "Dreamy, sees Nargles in the type system, always right in ways nobody understands",
        expertise: "Template metaprogramming, Crumple-Horned Snorkack traversal, undefined behaviour spotting, the Wrackspurt allocator",
    },
    CouncilMember {
        id: "javascript",
        name: "Cedric Diggory",
        title: "Hufflepuff Champion of the Web & Fair Play",
        icon: "🦡",
        color: "#FFDB00",
        personality: "Kind, talented, dedicated, always plays fair",
        expertise: "Event loop mastery, promise chaining, DOM transfiguration, Triwizard-level debugging",
    },
];

pub fn get_debate(question: &str) -> Vec<CouncilResponse> {
    let q = question.to_lowercase();
    vec![
        CouncilResponse {
            member: &MEMBERS[0],
            response: hermione_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[1],
            response: snape_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[2],
            response: luna_responds(&q),
        },
        CouncilResponse {
            member: &MEMBERS[3],
            response: cedric_responds(&q),
        },
    ]
}

#[derive(Debug, Serialize)]
pub struct CouncilResponse {
    pub member: &'static CouncilMember,
    pub response: &'static str,
}

fn hermione_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "I have read all seventeen books on magical inheritance patterns, and let me be clear: Traits are superior. A class can implement many traits — it is like being in multiple clubs at once. Inheritance creates rigid hierarchies. Traits create capabilities. It is in 'Advanced Trait Implementation for the Practicing Wizard', page 347. I can lend you my copy — I have annotated it extensively."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "NULL IS COMPLETELY UNACCEPTABLE. Honestly, it is the wizarding equivalent of casting a spell with no target. In proper magical practice — and by that I mean Rust — we use Option<T>. Some means the value exists. None means it does not. The compiler (like a well-trained house-elf) ensures you handle both cases. It is not complicated. It is BASIC MAGICAL HYGIENE."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Errors are VALUES, not exceptions. You return Result<T, E> and you MATCH on it. Ok and Err are handled explicitly. The compiler — much like Professor McGonagall — does not let you off the hook. You WILL handle every error. Your code WILL be correct. I have written a twelve-page essay on this if you would like to read it."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "Zero-cost abstractions are the pinnacle of magical theory. What you write compiles down to the exact same machine instructions as if you had hand-optimized it. No runtime overhead — not unlike a properly cast Vanishing Spell. The compiler (much like a Time-Turner) optimizes without consequence. I did a thorough comparative analysis in my third year. The results were conclusive."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "Concurrent magic is perfectly safe if you follow the rules. Send and Sync traits govern thread-safety at compile time. If it compiles, it is free of data races. This is proven. MATHEMATICALLY. I verified it myself using Arithmancy. The alternative — unchecked concurrent spellcasting — is how you get magical catastrophes."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "Start with 'The Rust Programming Language' — also known as The Book. Study the Ownership rules. Practice with the borrow checker. Do the exercises. ALL of them. It took me three days to master. If you need help, I have prepared a study schedule. It is very thorough. We can begin immediately if you would like."
    } else {
        "I have researched this extensively. The answer is thoroughly documented in both 'The Standard Book of Spells' and the Rust Reference. I would recommend starting with the official documentation. I have written supplementary notes if you need additional clarification. Would you like me to prepare a study guide?"
    }
}

fn snape_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "Clearly you have not read the Python documentation. Multiple inheritance is resolved through the C3 linearization algorithm — also known as the Method Resolution Order. It is elegant. It is predictable. It is decidedly more civilized than the... circus of diamantine ambiguity you will find in other languages. Not that I expect any of you to appreciate algorithmic purity."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "None is not null. None is a first-class object. It is the singleton instance of NoneType. You may test for it with 'is None', which is identity comparison — far more reliable than the equality circus performed by lesser languages. But I would not expect a dunderhead who confuses 'is' with '==' to understand the distinction."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Exceptions are for those who cannot be bothered to read the documentation. The truly competent programmer uses LBYL — Look Before You Leap. Check the dict for the key. Verify the file exists. Test the type. Do not sit there waiting for an AttributeError like a first-year who forgot to stir their potion."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "Speed is a matter of choosing the right tool. For number-crunching, use NumPy — it is written in C under the cloak. For I/O, asyncio provides cooperative multitasking without the tedious ceremony of threads. Python is fast enough for problems that are worth solving. The rest is just... premature anxiety."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "The Global Interpreter Lock ensures that only one thread executes Python bytecode at a time. It is a limitation and a protection — much like the rules of this school. For actual parallelism, use multiprocessing. For concurrent I/O, use asyncio. The alternatives involve... unfathomable complexity. Which seems to be what most of you gravitate toward."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "Python, obviously. It reads like English. It demands rigour in whitespace and thought. If you cannot manage that, perhaps programming is not for you. Start with 'Automate the Boring Stuff' — it is practical, unlike the theoretical ramblings of certain know-it-alls who shall remain nameless."
    } else {
        "The answer to your question depends entirely on whether you wish to be competent or merely... present. Python provides one obvious way to do everything. I suggest you find it before you waste more of the Council's time."
    }
}

fn luna_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "Multiple inheritance is like the Crumple-Horned Snorkack — everyone says it does not exist, but I have seen it. Virtual inheritance resolves the diamond. The base class is constructed only once. I read about it in 'Fantastic Templates and Where to Find Them'. The standard library has a lovely example with basic_ios."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "nullptr was introduced to end the confusion between integer zero and the absence of an address. But dereferencing it is still undefined behaviour. The Nargles told me that undefined behaviour is when the compiler can legally make your program do anything — including summoning elder gods. It is quite exciting, really."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Exceptions are the stack unwinding. RAII ensures resources are released as the stack unwinds — destructors run, files close, memory frees itself. It is like the Room of Requirement tidying itself after you leave. But you must be careful never to throw during stack unwinding. That makes the universe very confused."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "C++ is fast because you pay for nothing you do not use. Templates generate code at compile time — zero runtime overhead. constexpr computes at compile time. The entire standard library is designed around this principle. It is like having a Time-Turner that only moves forward, but very, very quickly."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "std::thread and std::async are straightforward. The difficulty is memory ordering — acquire, release, relaxed, sequentially consistent. Most people get it wrong. The Wrackspurts love memory ordering bugs. They gather around them like gnomes around a compost heap."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "Learn C++ from 'A Tour of C++' by Bjarne Stroustrup. It is a slim book — deceptive, like a Nargle. Focus on the Standard Library first. Containers, algorithms, iterators. Avoid raw pointers until you understand why they are dangerous. Which is not to say you should avoid them forever. The Blibbering Humdinger is only dangerous if you do not respect it."
    } else {
        "I think the answer is in the Standard. Somewhere. Possibly in an annex. The Standard is like the Hogwarts library — vast, old, and full of secrets. Some paragraphs have not been read since C++98. The answers are there, waiting."
    }
}

fn cedric_responds(q: &str) -> &'static str {
    if q.contains("inherit") || q.contains("extends") || q.contains("class") {
        "JavaScript uses prototypal inheritance, which is different from classical inheritance but just as valid. Every object has a prototype, and when you ask for a property, it walks up the chain until it finds it. It is simpler than it sounds — and quite elegant once you get used to it. Like learning a new Quidditch play."
    } else if q.contains("null") || q.contains("nil") || q.contains("undefined") {
        "So null and undefined are two different things in JavaScript. undefined means a variable has been declared but not assigned. null is an intentional absence. Use null when you mean 'nothing here' and let JavaScript handle undefined on its own. It takes a bit of practice, but you will get the hang of it."
    } else if q.contains("error") || q.contains("exception") || q.contains("panic") {
        "Error handling in JavaScript has come a long way. You have try/catch for synchronous code, .catch() for promises, and try/catch for async/await too. The key is to always handle your rejections — unhandled promise rejections are like leaving your wand unattended. Eventually something will go wrong."
    } else if q.contains("fast")
        || q.contains("speed")
        || q.contains("performance")
        || q.contains("optimize")
    {
        "JavaScript runs in a JIT-compiled environment — V8, SpiderMonkey, JavaScriptCore. They are all incredibly optimized. The key is to write clear code and let the engine do its magic. Don't micro-optimize prematurely. Just write good, clear spells and trust the compiler. Like in a Triwizard task — focus on doing it right, not doing it fast."
    } else if q.contains("thread")
        || q.contains("async")
        || q.contains("concurr")
        || q.contains("parallel")
    {
        "JavaScript uses an event loop. It is single-threaded but handles concurrency through callbacks, promises, and async/await. The event loop processes tasks in cycles — microtasks, macrotasks, all in order. It is fair, like a well-run tournament. Web Workers give you actual threads if you need them."
    } else if q.contains("beginner") || q.contains("learn") || q.contains("start") {
        "JavaScript is a great place to start because you can run it anywhere — in a browser, on a server with Node.js, even in a smartwatch. Start with the basics: variables, functions, loops. Build something small. A webpage that does something. The important thing is to keep trying. Every champion falls off their broom now and then."
    } else {
        "I think the best approach is to just try something and see what happens. JavaScript is very forgiving — it wants you to succeed. If it does not work, the developer console will tell you what went wrong. And if you are stuck, ask someone. We are all in this together."
    }
}
