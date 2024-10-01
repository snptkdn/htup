from textual.app import ComposeResult
from textual.binding import Binding
from textual.containers import Horizontal, Container
from textual.widgets import Static, RadioButton, RadioSet, Static
from typing import Optional

class PropertyDialog(Static):
    selected_method: Optional[RadioButton] = None

    BINDINGS = [
        Binding("ctrl+o", "send", "Send", priority=True)
    ]

    def compose(self) -> ComposeResult:
        with Horizontal():
            with RadioSet(id="method"):
                yield RadioButton("GET", id="radio_get")
                yield RadioButton("POST")
                yield RadioButton("PUT")
                yield RadioButton("DELETE")
            with RadioSet(id="contentType"):
                yield RadioButton("application/json")

    def on_radio_set_changed(self, changed):
        self.selected_method = changed.pressed
