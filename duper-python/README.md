<p align="center">
    <img src="https://duper.dev.br/logos/duper-400.png" alt="The Duper logo, with a confident spectacled mole wearing a flailing blue cape." /> <br>
</p>
<h1 align="center">duper-python</h1>

<p align="center">
    <a href="https://pypi.org/project/duper-python"><img alt="PyPI version" src="https://img.shields.io/pypi/v/duper-python?style=flat&logo=python&logoColor=white&label=duper-python"></a>
    <a href="https://github.com/EpicEric/duper"><img alt="GitHub license" src="https://img.shields.io/github/license/EpicEric/duper"></a>
</p>

Duper support for Python.

[Check out the official website for Duper.](https://duper.dev.br)

## Installation

```bash
uv add duper-python
# -- or --
pip install duper-python
```

## Examples

The basic `json`/`pickle`-like interface:

```python
import duper

DUPER_DATA = """
APIResponse({
  status: 200,
  headers: {
    content_type: "application/duper",
    cache_control: "max-age=3600",
  },
  body: {
    users: [
      User({
        id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"),
        name: "Alice",
        email: Email("alice@example.com"),
        roles: ["admin", "user"],
        metadata: {
          last_login: DateTime("2024-01-15T10:30:00Z"),
          ip: IPV4("173.255.230.79"),
        },
      }),
    ],
  },
})
"""

python_dict = duper.loads(DUPER_DATA)  # Actually a Pydantic BaseModel!

with open("out.duper", "w") as f:
    duper.dump(DUPER_DATA)
```

---

Using [Pydantic](https://pypi.org/project/pydantic/):

```python
from datetime import datetime
import re
import uuid

from duper import BaseModel


class RegisteredRegex(BaseModel):
    regex_id: uuid.UUID
    created_at: datetime
    pattern: re.Pattern
    matches: list[str] | None = None

data = RegisteredRegex(
    regex_id=uuid.uuid4(),
    created_at=datetime.now(),
    pattern=re.compile(r"^Hello w.rld!$"),
)

data_str = data.model_dump(mode="duper")
print(data_str)

reconstituted_data = RegisteredRegex.model_validate_duper(data_str)
assert data == reconstituted_data
```

---

Using [FastAPI](https://pypi.org/project/fastapi/):

```python
from typing import Annotated
from duper.fastapi import DuperBody, DuperResponse
from duper import BaseModel
from fastapi import FastAPI

class DuplicatableData(BaseModel):
    tup: tuple[str, bytes]
    value: int

app = FastAPI()

@app.post("/double", response_class=DuperResponse)
async def double_the_data(
    body: Annotated[DuplicatableData, DuperBody(DuplicatableData)],
) -> DuperResponse:
    return DuperResponse(
        DuplicatableData(
            tup=(body.tup[0] + body.tup[0], body.tup[1] + body.tup[1]),
            value=2 * body.value,
        )
    )
```
