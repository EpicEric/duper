from typing import Annotated
from duper.fastapi import DuperBody, DuperResponse
from duper.pydantic import BaseModel
from fastapi import FastAPI

class PydanticModel(BaseModel):
    tup: tuple[str, bytes]
    value: int


app = FastAPI()


@app.post("/response_pydantic", response_class=DuperResponse)
async def response_pydantic(
    body: Annotated[PydanticModel, DuperBody(PydanticModel)],
) -> DuperResponse:
    return DuperResponse(
        PydanticModel(
            tup=(body.tup[0] + body.tup[0], body.tup[1] + body.tup[1]),
            value=2 * body.value,
        )
    )


@app.get("/test", response_class=DuperResponse)
async def cool() -> DuperResponse:
    return DuperResponse(
        PydanticModel(
            tup=("test", b"123"),
            value=42,
        )
    )
