import logging
from db.qdrant import search_context
from db.postgres import get_or_create_session, get_history

logger = logging.getLogger(__name__)


def format_context(docs: list[str]) -> str:
    if not docs:
        return ""
    return "\n\n".join(f"[{i+1}] {d}" for i, d in enumerate(docs))


def format_history(messages: list[dict]) -> str:
    if not messages:
        return ""
    lines = []
    for m in messages:
        role = "Student" if m["role"] == "user" else m.get("agent_id", "Assistant")
        lines.append(f"{role}: {m['content']}")
    return "\n".join(lines)


async def memory_node(state: dict) -> dict:
    question = state.get("question", "")
    user_id = state.get("user_id", "anonymous")

    context_docs = await search_context(question)
    context = format_context(context_docs)
    if context:
        logger.info("Retrieved %d context docs for: %.60s", len(context_docs), question)

    history_messages = await get_history(user_id)
    history = format_history(history_messages)
    if history:
        logger.info("Retrieved %d history messages for user: %s", len(history_messages), user_id)

    return {
        "context": context,
        "history": history,
    }
