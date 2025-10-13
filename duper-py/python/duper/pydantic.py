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
    __doc__ = PydanticBaseModel.__doc__

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
