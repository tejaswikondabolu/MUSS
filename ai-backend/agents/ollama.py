import os
import logging

os.environ.setdefault("NO_PROXY", "localhost,127.0.0.1,::1")
os.environ.setdefault("no_proxy", "localhost,127.0.0.1,::1")

import httpx

OLLAMA_BASE = os.getenv("OLLAMA_URL", "http://localhost:11434").rstrip("/")
LLM_MODEL = os.getenv("LLM_MODEL", "gemma3:4b")
EMBED_MODEL = os.getenv("EMBED_MODEL", "nomic-embed-text")

logger = logging.getLogger(__name__)

_client: httpx.AsyncClient | None = None


def _get_client() -> httpx.AsyncClient:
    global _client
    if _client is None:
        _client = httpx.AsyncClient(
            timeout=httpx.Timeout(300.0, connect=10.0),
            limits=httpx.Limits(max_keepalive_connections=4, max_connections=4),
        )
    return _client


async def call_ollama(system_prompt: str, user_message: str, temperature: float = 0.7) -> str:
    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_message},
    ]
    client = _get_client()
    resp = await client.post(f"{OLLAMA_BASE}/api/chat", json={
        "model": LLM_MODEL,
        "messages": messages,
        "stream": False,
        "temperature": temperature,
        "max_tokens": 1024,
    })
    resp.raise_for_status()
    return resp.json()["message"]["content"]


async def get_embedding(text: str) -> list[float]:
    client = _get_client()
    resp = await client.post(f"{OLLAMA_BASE}/api/embeddings", json={
        "model": EMBED_MODEL,
        "prompt": text,
    })
    resp.raise_for_status()
    return resp.json()["embedding"]
