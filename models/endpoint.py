import json

import pydantic
from variables.method import Method


class Endpoint(pydantic.BaseModel):
    method: Method
    url: str

def load_endpoint(path):
    with open(path) as f:
        data = json.load(f)
        return Endpoint.model_validate(data)
