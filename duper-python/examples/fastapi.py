from typing import Annotated
from duper.fastapi import DuperBody, DuperResponse
from duper import BaseModel
from fastapi import FastAPI


class CustomData(BaseModel):
    tup: tuple[str, bytes]
    value: int


app = FastAPI()


@app.post("/response_pydantic", response_class=DuperResponse)
async def response_pydantic(
    body: Annotated[CustomData, DuperBody(CustomData)],
) -> DuperResponse:
    return DuperResponse(
        CustomData(
            tup=(body.tup[0] + body.tup[0], body.tup[1] + body.tup[1]),
            value=2 * body.value,
        )
    )


@app.get("/test", response_class=DuperResponse)
async def cool() -> DuperResponse:
    return DuperResponse(
        CustomData(
            tup=("test", b"123"),
            value=42,
        )
    )
