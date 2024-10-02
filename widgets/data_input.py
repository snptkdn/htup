from textual.app import App, ComposeResult
from textual.widgets import Static, TextArea

class DataInput(Static):
    def compose(self) -> ComposeResult:
        yield TextArea.code_editor("", language="json", id="data_input")
