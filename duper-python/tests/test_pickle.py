import pickle
import duper


class MyClass:
    def __init__(self, name: str):
        self.name = name

    def greet(self) -> str:
        return f"Hello, {self.name}!"


def test_pickle():
    obj = MyClass("McDuper")
    pic = pickle.dumps(obj)
    dup = duper.dumps(pic)

    assert type(dup) is str
    assert dup.startswith('b"')
    assert dup.endswith('"')

    undup = duper.loads(dup, parse_any=True)
    unpic = pickle.loads(undup)

    assert unpic.greet() == "Hello, McDuper!"
