import logging
from contextlib import asynccontextmanager

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from schemas.models import CouncilAskRequest, CouncilAskResponse, CouncilMemberResponse
from agents.planner import council_graph
from db.postgres import init_pool, close_pool, get_or_create_session, save_message
from db.qdrant import ensure_collection

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(name)s: %(message)s")
logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("Starting up — initializing DB connections...")
    await init_pool()
    await ensure_collection()
    yield
    logger.info("Shutting down — closing DB connections...")
    await close_pool()


app = FastAPI(title="Hogwarts AI Orchestrator", version="0.3.0", lifespan=lifespan)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health():
    return {"status": "🏰 Hogwarts AI Backend is live"}


@app.post("/council/ask", response_model=CouncilAskResponse)
async def council_ask(req: CouncilAskRequest):
    if not req.question.strip():
        raise HTTPException(status_code=400, detail="Question cannot be empty")

    session_id = await get_or_create_session(req.user_id)
    if session_id:
        await save_message(session_id, "user", req.question)

    result = await council_graph.ainvoke({
        "question": req.question,
        "user_id": req.user_id,
        "context": "",
        "history": "",
    })
    responses = result.get("responses", [])

    if session_id:
        for r in responses:
            await save_message(session_id, "assistant", r["response"], r["id"])

    return CouncilAskResponse(responses=[
        CouncilMemberResponse(**r) for r in responses
    ])


if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8001, reload=True)
