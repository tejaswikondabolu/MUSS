from pydantic import BaseModel


class CouncilAskRequest(BaseModel):
    question: str
    user_id: str = "anonymous"


class CouncilMemberResponse(BaseModel):
    id: str
    name: str
    title: str
    icon: str
    color: str
    personality: str
    expertise: str
    response: str


class CouncilAskResponse(BaseModel):
    responses: list[CouncilMemberResponse]
