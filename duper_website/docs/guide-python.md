# Python guide

Get started with Duper in Python with the [`duper-python`](https://pypi.org/project/duper-python/) package.

## Installation

```bash
uv add duper-python
```

## Quick example

```python
import duper

data = {
    "name": "Wireless Headphones",
    "price": 129.99,
    "in_stock": True,
    "tags": ["electronics", "audio"]
}

# Convert to Duper format
duper_string = duper.dumps(data)
print(duper_string)

# Convert back from Duper
restored_data = duper.loads(duper_string)
```

## Interaction with files

```python
import duper

# Simple serialization
data = {"name": "Alice", "age": 30, "active": True}
duper_string = duper.dumps(data)

# Write to file
with open("data.duper", "w") as f:
    duper.dump(data, f)

# Read from file
with open("data.duper", "r") as f:
    loaded_data = duper.load(f)
```

## Using Pydantic models

`duper-python` comes with full type validation and serialization for Pydantic:

```python
from duper import BaseModel, Duper
from datetime import datetime
import uuid

class Product(BaseModel):
    id: uuid.UUID
    name: str
    price: Annotated[float, Duper("Dollars")]
    created_at: datetime

# Create and serialize
product = Product(
    id=uuid.uuid4(),
    name="Wireless Headphones",
    price=129.99,
    created_at=datetime.now()
)

# Convert to Duper string
duper_string = product.model_dump(mode="duper")
print(duper_string)

# Convert back from Duper
restored = Product.model_validate_duper(duper_string)
```

## FastAPI integration

With FastAPI, we have built-in support for requests and responses:

```python
from duper import BaseModel
from duper.fastapi import DuperBody, DuperResponse
from fastapi import FastAPI

class Item(BaseModel):
    name: str
    value: int

app = FastAPI()

@app.post("/double", response_class=DuperResponse)
async def double_item(
    item: Item = DuperBody(Item)
) -> DuperResponse:
    doubled = Item(name=item.name * 2, value=item.value * 2)
    return DuperResponse(doubled)
```
