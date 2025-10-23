from collections.abc import Mapping
from typing import Any, TypeVar
from typing_extensions import override

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
    """
    An HTTP response containing a Duper value.

    >>> import FastAPI
    >>> from duper.fastapi import DuperResponse
    >>> app = FastAPI()
    >>> @app.get("/")
    ... async def duper_response() -> DuperResponse:
    ...     return DuperResponse({
    ...         "success": true,
    ...     })
    """

    media_type: str = DUPER_CONTENT_TYPE
    _indent: int | None
    _strip_identifiers: bool

    def __init__(
        self,
        content: Any,  # pyright: ignore[reportExplicitAny, reportAny]
        status_code: int = 200,
        headers: Mapping[str, str] | None = None,
        background: BackgroundTask | None = None,
        indent: int | None = None,
        strip_identifiers: bool = False,
    ) -> None:
        self._indent = indent
        self._strip_identifiers = strip_identifiers
        super().__init__(content, status_code, headers, self.media_type, background)

    @override
    def render(self, content: Any) -> bytes:  # pyright: ignore[reportExplicitAny, reportAny]
        return dumps(
            content,
            indent=self._indent,
            strip_identifiers=self._strip_identifiers,
        ).encode("utf-8")


def DuperBody(model_type: type[T]) -> Any:  # pyright: ignore[reportExplicitAny, reportAny]
    """
    A dependency providing automatic parsing of an HTTP request containing a Duper value.

    >>> from typing import Any
    >>> import FastAPI
    >>> from duper.fastapi import DuperBody
    >>> app = FastAPI()
    >>> @app.post("/")
    ... async def duper_body(
    ...     body: Annotated[dict[str, Any], DuperBody(dict[str, Any])],
    ... ):
    ...     print(body)
    """

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
        parsed = loads(body.decode(encoding="utf-8"), parse_any=True)
        if isinstance(parsed, PydanticBaseModel):
            dumped = parsed.model_dump(mode="python")
        else:
            adapter = TypeAdapter(type(parsed))
            dumped = adapter.dump_python(parsed)  # pyright: ignore[reportAny]

        if issubclass(model_type, PydanticBaseModel):
            return model_type.model_validate(dumped)
        try:
            adapter = TypeAdapter(model_type)
            return adapter.validate_python(dumped)
        except Exception:
            return dumped  # pyright: ignore[reportReturnType]

    return Depends(_get_duper_body)  # pyright: ignore[reportAny]
