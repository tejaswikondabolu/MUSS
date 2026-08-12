from typing import TypedDict, Annotated
from langgraph.graph import StateGraph, END
from langgraph.types import Send
from agents import hermione, snape, luna, cedric, memory


def concat_responses(a: list | None, b: list | None) -> list:
    if a is None:
        return b or []
    if b is None:
        return a
    return a + b


class CouncilState(TypedDict):
    question: str
    user_id: str
    context: str
    history: str
    responses: Annotated[list, concat_responses]


def router(state: CouncilState) -> list[Send]:
    q = state["question"].lower()
    targets = ["hermione"]
    if any(w in q for w in ["python", "indentation", "list", "dict", "flask", "django", "pip"]):
        targets.append("snape")
    if any(w in q for w in ["c++", "cpp", "pointer", "template", "memory", "undefined", "raii"]):
        targets.append("luna")
    if any(w in q for w in ["javascript", "js", "async", "promise", "browser", "dom", "node"]):
        targets.append("cedric")
    if len(targets) == 1:
        targets.extend(["snape", "luna", "cedric"][:2])
    return [
        Send(t, {
            "question": state["question"],
            "context": state.get("context", ""),
            "history": state.get("history", ""),
        })
        for t in targets
    ]


async def memory_node(state: CouncilState) -> dict:
    return await memory.memory_node(state)


async def hermione_node(state: CouncilState) -> dict:
    results = await hermione.node(
        state["question"],
        context=state.get("context", ""),
        history=state.get("history", ""),
    )
    return {"responses": results}


async def snape_node(state: CouncilState) -> dict:
    results = await snape.node(
        state["question"],
        context=state.get("context", ""),
        history=state.get("history", ""),
    )
    return {"responses": results}


async def luna_node(state: CouncilState) -> dict:
    results = await luna.node(
        state["question"],
        context=state.get("context", ""),
        history=state.get("history", ""),
    )
    return {"responses": results}


async def cedric_node(state: CouncilState) -> dict:
    results = await cedric.node(
        state["question"],
        context=state.get("context", ""),
        history=state.get("history", ""),
    )
    return {"responses": results}


def build_council_graph():
    builder = StateGraph(CouncilState)
    builder.add_node("memory", memory_node)
    builder.add_node("hermione", hermione_node)
    builder.add_node("snape", snape_node)
    builder.add_node("luna", luna_node)
    builder.add_node("cedric", cedric_node)
    builder.add_edge("__start__", "memory")
    builder.add_conditional_edges(
        "memory",
        router,
        ["hermione", "snape", "luna", "cedric"],
    )
    for name in ["hermione", "snape", "luna", "cedric"]:
        builder.add_edge(name, END)
    return builder.compile()


council_graph = build_council_graph()
