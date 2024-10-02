import os
from os.path import expanduser
from pathlib import Path
from pathlib import PosixPath
from textual.app import App, ComposeResult
from textual.containers import Center
from textual.containers import Grid
from textual.screen import ModalScreen
from textual.widgets import Button, Footer, Header, Input, Static, Label

from models.endpoint import Endpoint

class NewEndpointModal(ModalScreen):
    """Screen with a dialog to quit."""

    def __init__(self, path: PosixPath, node, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.path: PosixPath = path
        self.node = node

    def compose(self) -> ComposeResult:
        yield Grid(
            Label("Input New Endpoint Name"),
            Input(placeholder="new endpoint name", id="input"),
            Button("OK", variant="primary", id="ok"),
            Button("Cancel", variant="primary", id="cancel"),
            id="dialog",
        )
    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "ok":
            file_path = self.path / self.query_one('#input').value
            Path(file_path).touch()
            with open(file_path, 'w') as file:
                file.write(Endpoint(method="get", url="", data="").model_dump_json())
            self.app.pop_screen()
        else:
            self.app.pop_screen()
