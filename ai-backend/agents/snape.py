from agents.ollama import call_ollama

MEMBER = {
    "id": "python",
    "name": "Professor Severus Snape",
    "title": "Master of Potions & Practical Incantations",
    "icon": "⚗️",
    "color": "#2ecc71",
    "personality": "Profound, bitter, powerful, tolerates no foolishness",
    "expertise": "Potion-brewing (pipeline design), occlumency (encapsulation), the Dark Arts (metaprogramming), Legilimency (introspection)",
}

with open("prompts/snape.txt") as f:
    SYSTEM_PROMPT = f.read()


async def node(question: str, context: str = "", history: str = "") -> list[dict]:
    system = SYSTEM_PROMPT
    if context:
        system += f"\n\nRelevant context from the archives:\n{context}"
    if history:
        system += f"\n\nRecent conversation history:\n{history}"
    user_message = (
        f"A student has asked the Council of Hogwarts the following question. "
        f"Respond as Professor Severus Snape, Master of Potions & Practical Incantations.\n\n"
        f"Question: {question}\n\n"
        f"Remember: Stay in character. You are Severus Snape, not an AI. "
        f"Explain programming concepts through Hogwarts metaphors. Be sarcastic and condescending. You have no patience for foolishness."
    )
    reply = await call_ollama(system, user_message)
    return [{**MEMBER, "response": reply}]
