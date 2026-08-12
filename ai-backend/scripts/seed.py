"""
Seed Qdrant with documentation chunks for RAG context.

Usage:
    python -m scripts.seed

Requires:
    - Qdrant running on QDRANT_URL (default: http://localhost:6333)
    - Ollama running with EMBED_MODEL (default: nomic-embed-text)
"""
import asyncio
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from db.qdrant import upsert_docs, ensure_collection


async def main():
    seed_path = os.path.join(os.path.dirname(__file__), "seed_data.json")
    with open(seed_path) as f:
        docs = json.load(f)

    print(f"Loading {len(docs)} seed documents...")
    await ensure_collection()
    await upsert_docs(docs)
    print("Done! Seed data uploaded to Qdrant.")


if __name__ == "__main__":
    asyncio.run(main())
