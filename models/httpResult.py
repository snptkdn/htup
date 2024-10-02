from dataclasses import dataclass
from typing import Union


@dataclass()
class HttpResult():
    content_type: str
    status_code: int
    body: Union[str, object]
    time: float
