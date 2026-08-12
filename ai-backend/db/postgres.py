import os
import uuid
import logging
import asyncpg

DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://hogwarts:hogwarts@localhost:5432/hogwarts")

logger = logging.getLogger(__name__)

_pool: asyncpg.Pool | None = None


async def init_pool():
    global _pool
    try:
        _pool = await asyncpg.create_pool(DATABASE_URL, min_size=1, max_size=4)
        logger.info("PostgreSQL pool created")
    except Exception as e:
        logger.warning("PostgreSQL unavailable: %s", e)


async def close_pool():
    global _pool
    if _pool:
        await _pool.close()
        _pool = None
        logger.info("PostgreSQL pool closed")


async def get_or_create_session(user_id: str) -> str | None:
    if not _pool:
        return None
    try:
        async with _pool.acquire() as conn:
            row = await conn.fetchrow(
                "SELECT id FROM sessions WHERE user_id = $1 ORDER BY updated_at DESC LIMIT 1",
                user_id,
            )
            if row:
                session_id = row["id"]
                await conn.execute(
                    "UPDATE sessions SET updated_at = NOW() WHERE id = $1",
                    session_id,
                )
                return str(session_id)
            session_id = uuid.uuid4()
            await conn.execute(
                "INSERT INTO sessions (id, user_id) VALUES ($1, $2)",
                session_id, user_id,
            )
            return str(session_id)
    except Exception as e:
        logger.warning("Failed to get/create session: %s", e)
        return None


async def save_message(session_id: str, role: str, content: str, agent_id: str | None = None):
    if not _pool:
        return
    try:
        async with _pool.acquire() as conn:
            await conn.execute(
                "INSERT INTO messages (session_id, role, content, agent_id) VALUES ($1, $2, $3, $4)",
                session_id, role, content, agent_id,
            )
    except Exception as e:
        logger.warning("Failed to save message: %s", e)


async def get_history(user_id: str, limit: int = 6) -> list[dict]:
    if not _pool:
        return []
    try:
        async with _pool.acquire() as conn:
            rows = await conn.fetch(
                """SELECT m.role, m.content, m.agent_id, m.created_at
                   FROM messages m
                   JOIN sessions s ON m.session_id = s.id
                   WHERE s.user_id = $1
                   ORDER BY m.created_at DESC
                   LIMIT $2""",
                user_id, limit,
            )
            return [
                {
                    "role": r["role"],
                    "content": r["content"],
                    "agent_id": r["agent_id"],
                }
                for r in reversed(rows)
            ]
    except Exception as e:
        logger.warning("Failed to get history: %s", e)
        return []
