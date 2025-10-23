import sys
from typing import Any, Callable, TypeVar, cast
from pydantic.config import ExtraValues
from typing_extensions import Self

from pydantic import (
    ConfigDict,
    ValidationError,
    model_serializer,
    BaseModel as PydanticBaseModel,
    create_model as _create_model,
    SerializationInfo,
    SerializerFunctionWrapHandler,
)

__all__ = [
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
            from ._duper import dumps

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
            from ._duper import loads

            loaded = loads(serialized, parse_any=False)
            if isinstance(loaded, list):
                raise ValidationError("cannot validate Duper list")
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


ModelT = TypeVar("ModelT", bound=BaseModel)


def create_model(
    model_name: str,
    /,
    *,
    __config__: ConfigDict | None = None,
    __doc__: str | None = None,
    __base__: type[ModelT] | tuple[type[ModelT], ...] | None = None,
    __module__: str | None = None,
    __validators__: dict[str, Callable[..., Any]] | None = None,  # pyright: ignore[reportExplicitAny]
    __cls_kwargs__: dict[str, Any] | None = None,  # pyright: ignore[reportExplicitAny]
    __qualname__: str | None = None,
    **field_definitions: Any | tuple[str, Any],  # pyright: ignore[reportExplicitAny]
) -> type[ModelT]:
    """Dynamically creates and returns a new Pydantic model, in other words,
    ``create_model`` dynamically creates a subclass of ``BaseModel``.

    Args:
        model_name: The name of the newly created model.
        __config__: The configuration of the new model.
        __doc__: The docstring of the new model.
        __base__: The base class or classes for the new model.
        __module__: The name of the module that the model belongs to;
            if `None`, the value is taken from ``sys._getframe(1)``
        __validators__: A dictionary of methods that validate fields. The keys
            are the names of the validation methods to be added to the model,
            and the values are the validation methods themselves. You can read
            more about functional validators [here](https://docs.pydantic.dev/2.9/concepts/validators/#field-validators).
        __cls_kwargs__: A dictionary of keyword arguments for class creation,
            such as `metaclass`.
        __qualname__: The qualified name of the newly created model.
        **field_definitions: Field definitions of the new model. Either:

        - a single element, representing the type annotation of the field.
        - a two-tuple, the first element being the type and the second
            element the assigned value (either a default or the ``Field()``
            function).

    Returns:
        The new ``BaseModel``.

    Raises:
        PydanticUserError: If ``__base__`` and ``__config__`` are both passed.
    """

    if __base__ is None:
        __base__ = (cast("type[ModelT]", BaseModel),)

    if __module__ is None:
        f = sys._getframe(1)  # pyright: ignore[reportPrivateUsage]
        __module__ = cast(str, f.f_globals["__name__"])

    return _create_model(
        model_name,
        __config__=__config__,
        __doc__=__doc__,
        __base__=__base__,
        __module__=__module__,
        __validators__=__validators__,
        __cls_kwargs__=__cls_kwargs__,
        __qualname__=__qualname__,
        **field_definitions,
    )
