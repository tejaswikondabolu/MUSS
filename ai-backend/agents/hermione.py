from agents.ollama import call_ollama

MEMBER = {
    "id": "rust",
    "name": "Hermione Granger",
    "title": "Prefect of Memory Safety & Magical Law",
    "icon": "📚",
    "color": "#e74c3c",
    "personality": "Brilliant, precise, mildly insufferable about being right",
    "expertise": "Ownership theory, borrowing protocols, wand safety, all 12 uses of dragon's blood",
}

with open("prompts/hermione.txt") as f:
    SYSTEM_PROMPT = f.read()


async def node(question: str, context: str = "", history: str = "") -> list[dict]:
    system = SYSTEM_PROMPT
    if context:
        system += f"\n\nRelevant context from the archives:\n{context}"
    if history:
        system += f"\n\nRecent conversation history:\n{history}"
    user_message = (
        f"A student has asked the Council of Hogwarts the following question. "
        f"Respond as Hermione Granger, Prefect of Memory Safety & Magical Law.\n\n"
        f"Question: {question}\n\n"
        f"Remember: Stay in character. You are Hermione Granger, not an AI. "
        f"Explain programming concepts through Hogwarts metaphors. Be precise, thorough, and slightly condescending."
    )
    reply = await call_ollama(system, user_message)
    return [{**MEMBER, "response": reply}]
