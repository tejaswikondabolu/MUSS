import os
import logging
from qdrant_client import QdrantClient
from qdrant_client.http import models
from agents.ollama import get_embedding

QDRANT_URL = os.getenv("QDRANT_URL", "http://localhost:6333")
COLLECTION_NAME = "hogwarts_lore"
VECTOR_SIZE = 768

logger = logging.getLogger(__name__)

_client: QdrantClient | None = None


def get_client() -> QdrantClient:
    global _client
    if _client is None:
        _client = QdrantClient(url=QDRANT_URL)
    return _client


async def ensure_collection():
    try:
        client = get_client()
        collections = client.get_collections().collections
        if COLLECTION_NAME not in [c.name for c in collections]:
            client.create_collection(
                collection_name=COLLECTION_NAME,
                vectors_config=models.VectorParams(
                    size=VECTOR_SIZE,
                    distance=models.Distance.COSINE,
                ),
            )
            logger.info("Created Qdrant collection: %s", COLLECTION_NAME)
    except Exception as e:
        logger.warning("Qdrant unavailable: %s", e)


async def search_context(query: str, top_k: int = 3) -> list[str]:
    try:
        embedding = await get_embedding(query)
        client = get_client()
        results = client.search(
            collection_name=COLLECTION_NAME,
            query_vector=embedding,
            limit=top_k,
        )
        return [r.payload["text"] for r in results if r.payload]
    except Exception as e:
        logger.warning("Qdrant search failed: %s", e)
        return []


async def upsert_docs(docs: list[dict]):
    if not docs:
        return
    try:
        await ensure_collection()
        client = get_client()
        points = []
        for doc in docs:
            embedding = await get_embedding(doc["text"])
            points.append(models.PointStruct(
                id=doc["id"],
                vector=embedding,
                payload={
                    "text": doc["text"],
                    "source": doc.get("source", ""),
                    "language": doc.get("language", ""),
                    "character_id": doc.get("character_id", ""),
                },
            ))
        client.upsert(collection_name=COLLECTION_NAME, points=points)
        logger.info("Upserted %d docs to Qdrant", len(points))
    except Exception as e:
        logger.error("Qdrant upsert failed: %s", e)
