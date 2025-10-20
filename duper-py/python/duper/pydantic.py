from typing_extensions import Self

from pydantic import (
    model_serializer,
    BaseModel as PydanticBaseModel,
    SerializationInfo,
    SerializerFunctionWrapHandler,
)

from ._duper import dumps, loads

__all__ = [
    "BaseModel",
]


class BaseModel(PydanticBaseModel):
    """
    A wrapper around Pydantic's BaseModel with added functionality for
    serializing/deserializing Duper values.

    In order to serialize an instance of this model:

    >>> from duper.pydantic import BaseModel
    >>> class Foo(BaseModel):
    ...     bar: str
    ...
    >>> obj = Foo(bar="duper")
    >>> s = obj.model_dump(mode="duper")
    >>> print(s)
    Foo({bar: "duper"})

    In order to deserialize a string containing a Duper value:

    >>> from duper.pydantic import BaseModel
    >>> class Foo(BaseModel):
    ...     bar: str
    ...
    >>> s = "Foo({bar: \"duper\"})"
    >>> obj = Foo.model_validate_duper(s)
    >>> obj
    Foo(bar='duper')
    """

    @model_serializer(mode="wrap")
    def serialize_model(
        self,
        handler: SerializerFunctionWrapHandler,
        info: SerializationInfo,
        *,
        strip_identifiers: bool = False,
    ) -> dict[str, object] | str:
        if info.mode == "duper":
            return dumps(self, strip_identifiers=strip_identifiers)
        return handler(self)

    @classmethod
    def model_validate_duper(cls, serialized: str | bytes | object) -> Self:
        if type(serialized) is bytes:
            serialized = serialized.decode(encoding="utf-8")
        if type(serialized) is str:
            return cls.model_validate(loads(serialized))
        return cls.model_validate(serialized)
