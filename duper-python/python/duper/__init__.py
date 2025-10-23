r"""Utilities for converting to and from Python types into the Duper format.

:mod:`duper` exposes an API similar to :mod:`json` and :mod:`pickle`."""

from typing import Any
from pydantic.config import ExtraValues
from typing_extensions import Self

from pydantic import (
    model_serializer,
    BaseModel as PydanticBaseModel,
    SerializationInfo,
    SerializerFunctionWrapHandler,
)

from ._duper import (
    dumps,
    dump,
    loads,
    load,
    Duper,
    DuperType,
)

__all__ = [
    "dumps",
    "dump",
    "loads",
    "load",
    "Duper",
    "DuperType",
    "BaseModel",
]


class BaseModel(PydanticBaseModel):
    """
    A wrapper around Pydantic's BaseModel with added functionality for
    serializing/deserializing Duper values.

    In order to serialize an instance of this model:

    >>> from duper import BaseModel
    >>> class Foo(BaseModel):
    ...     bar: str
    ...
    >>> obj = Foo(bar="duper")
    >>> s = obj.model_dump(mode="duper")
    >>> print(s)
    Foo({bar: "duper"})

    In order to deserialize a string containing a Duper value:

    >>> from duper import BaseModel
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
        return handler(self)  # pyright: ignore[reportAny]

    @classmethod
    def model_validate_duper(
        cls,
        serialized: str | bytes | object,
        *,
        strict: bool | None = None,
        extra: ExtraValues | None = None,
        from_attributes: bool | None = None,
        context: Any | None = None,  # pyright: ignore[reportExplicitAny]
        by_alias: bool | None = None,
        by_name: bool | None = None,
    ) -> Self:
        """Validate a Pydantic model instance.

        Args:
            serialized: The object to validate.
            strict: Whether to enforce types strictly.
            extra: Whether to ignore, allow, or forbid extra data during model validation.
                See the [`extra` configuration value][pydantic.ConfigDict.extra] for details.
            from_attributes: Whether to extract data from object attributes.
            context: Additional context to pass to the validator.
            by_alias: Whether to use the field's alias when validating against the provided input data.
            by_name: Whether to use the field's name when validating against the provided input data.

        Raises:
            ValidationError: If the object could not be validated.

        Returns:
            The validated model instance.
        """
        if type(serialized) is bytes:
            serialized = serialized.decode(encoding="utf-8")

        if type(serialized) is str:
            loaded = loads(serialized)
            if isinstance(loaded, list):
                raise ValueError("cannot validate Duper list")
            return cls.model_validate(
                loaded.model_dump(mode="python"),
                strict=strict,
                extra=extra,
                from_attributes=from_attributes,
                context=context,
                by_alias=by_alias,
                by_name=by_name,
            )

        return cls.model_validate(
            serialized,
            strict=strict,
            extra=extra,
            from_attributes=from_attributes,
            context=context,
            by_alias=by_alias,
            by_name=by_name,
        )
