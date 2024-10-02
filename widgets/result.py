from textual.app import ComposeResult
from textual.containers import Container, Horizontal, ScrollableContainer
from textual.widgets import Label, Pretty, Static

from requester import http

class Result(Static):
    def compose(self) -> ComposeResult:
        with ScrollableContainer():
            with Container(classes="result_component"):
                yield Label("Status Code")
                yield Pretty("-", id="status_code", classes="result_label")
            with Container(classes="result_component"):
                yield Label("Content Type")
                yield Pretty("-", id="content_type", classes="result_label")
            with Container(classes="result_component"):
                yield Label("Response Time")
                yield Pretty("-", id="response_time", classes="result_label")
            with Container(classes="result_component"):
                yield Label("ResponseBody")
                yield Pretty("Introduction", id="intro", classes="result_label")
