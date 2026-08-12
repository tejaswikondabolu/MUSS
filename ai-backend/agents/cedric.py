from agents.ollama import call_ollama

MEMBER = {
    "id": "javascript",
    "name": "Cedric Diggory",
    "title": "Hufflepuff Champion of the Web & Fair Play",
    "icon": "🦡",
    "color": "#FFDB00",
    "personality": "Kind, talented, dedicated, always plays fair",
    "expertise": "Event loop mastery, promise chaining, DOM transfiguration, Triwizard-level debugging",
}

with open("prompts/cedric.txt") as f:
    SYSTEM_PROMPT = f.read()


async def node(question: str, context: str = "", history: str = "") -> list[dict]:
    system = SYSTEM_PROMPT
    if context:
        system += f"\n\nRelevant context from the archives:\n{context}"
    if history:
        system += f"\n\nRecent conversation history:\n{history}"
    user_message = (
        f"A student has asked the Council of Hogwarts the following question. "
        f"Respond as Cedric Diggory, Hufflepuff Champion of the Web & Fair Play.\n\n"
        f"Question: {question}\n\n"
        f"Remember: Stay in character. You are Cedric Diggory, not an AI. "
        f"Explain programming concepts through Hogwarts metaphors. Be warm, encouraging, and humble. "
        f"Reference Quidditch, the Triwizard Tournament, or Hufflepuff values naturally."
    )
    reply = await call_ollama(system, user_message)
    return [{**MEMBER, "response": reply}]
