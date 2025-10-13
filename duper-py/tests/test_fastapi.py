from typing import Annotated, Any
from fastapi import FastAPI, HTTPException
from fastapi.testclient import TestClient

from duper.fastapi import DuperResponse, DuperBody
from duper.pydantic import BaseModel


app = FastAPI()


class PydanticModel(BaseModel):
    tup: tuple[str, bytes]


@app.get("/response_dict")
async def response_dict() -> DuperResponse:
    return DuperResponse(
        {"duper": (1, 2.0, None, ["FastAPI", True]), "bytes": b"12345"}
    )


@app.get("/response_pydantic")
async def response_pydantic() -> DuperResponse:
    return DuperResponse(PydanticModel(tup=("hello", b"world")))


@app.post("/body_dict")
async def body_dict(
    body: Annotated[dict[str, Any], DuperBody(dict[str, Any])],
) -> dict[str, bool]:
    if body == {"duper": (1, 2.0, None, ["FastAPI", True]), "bytes": b"12345"}:
        return {"success": True}
    raise HTTPException(status_code=400, detail=body)


@app.post("/body_pydantic")
async def body_pydantic(
    body: Annotated[PydanticModel, DuperBody(PydanticModel)],
) -> dict[str, bool]:
    if body == PydanticModel(tup=("hello", b"world")):
        return {"success": True}
    raise HTTPException(status_code=400, detail=body)


client = TestClient(app)


def test_duper_response_dict():
    response = client.get("/response_dict")
    assert response.status_code == 200
    assert response.headers.get("Content-Type") == "application/duper"
    assert (
        response.text
        == r"""{duper: (1, 2.0, null, ["FastAPI", true]), bytes: b"12345"}"""
    )


def test_duper_response_pydantic():
    response = client.get("/response_pydantic")
    assert response.status_code == 200
    assert response.headers.get("Content-Type") == "application/duper"
    assert response.text == r"""Pydanticmodel({tup: ("hello", b"world")})"""


def test_duper_body_dict():
    response = client.post(
        "/body_dict",
        content=r"""{duper: (1, 2.0, null, ["FastAPI", true]), bytes: b"12345"}""",
        headers={"Content-Type": "application/duper"},
    )
    assert response.status_code == 200
    assert response.json() == {"success": True}


def test_duper_body_pydantic():
    response = client.post(
        "/body_pydantic",
        content=r"""{tup: ("hello", b"world")}""",
        headers={"Content-Type": "application/duper"},
    )
    assert response.status_code == 200
    assert response.json() == {"success": True}
