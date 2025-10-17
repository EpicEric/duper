from collections.abc import Mapping
from typing import Any, TypeVar

from fastapi import Depends, HTTPException, Request, status
from pydantic import BaseModel as PydanticBaseModel, TypeAdapter
from starlette.background import BackgroundTask
from starlette.responses import Response

from ._duper import dumps, loads

__all__ = [
    "DuperBody",
    "DuperResponse",
]

DUPER_CONTENT_TYPE = "application/duper"
DUPER_ALT_CONTENT_TYPE = "application/x-duper"


T = TypeVar("T")


class DuperResponse(Response):
    media_type = DUPER_CONTENT_TYPE
    _indent: int | None
    _strip_identifiers: bool

    def __init__(
        self,
        content: Any,
        status_code: int = 200,
        headers: Mapping[str, str] | None = None,
        background: BackgroundTask | None = None,
        indent: int | None = None,
        strip_identifiers: bool = False,
    ) -> None:
        self._indent = indent
        self._strip_identifiers = strip_identifiers
        super().__init__(content, status_code, headers, self.media_type, background)

    def render(self, content: Any) -> bytes:
        return dumps(
            content,
            indent=self._indent,
            strip_identifiers=self._strip_identifiers,
        ).encode("utf-8")


def DuperBody(model_type: type[T]) -> Any:
    async def _get_duper_body(request: Request) -> T:
        if request.headers.get("Content-Type") not in (
            DUPER_CONTENT_TYPE,
            DUPER_ALT_CONTENT_TYPE,
        ):
            raise HTTPException(
                status_code=status.HTTP_415_UNSUPPORTED_MEDIA_TYPE,
                detail=f"Content-Type header must be {DUPER_CONTENT_TYPE}",
            )

        body = await request.body()
        parsed = loads(body.decode(encoding="utf-8"))

        if issubclass(model_type, PydanticBaseModel):
            return model_type.model_validate(parsed)
        try:
            adapter = TypeAdapter(model_type)
            return adapter.validate_python(parsed)
        except Exception:
            return parsed

    return Depends(_get_duper_body)
