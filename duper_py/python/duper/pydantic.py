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
    @model_serializer(mode="wrap")
    def serialize_model(
        self, handler: SerializerFunctionWrapHandler, info: SerializationInfo
    ) -> dict[str, object] | str:
        if info.mode == "duper":
            return dumps(self)
        return handler(self)

    @classmethod
    def model_validate_duper(cls, string: str) -> Self:
        return cls.model_validate(loads(string))
