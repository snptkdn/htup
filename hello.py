import os
from os.path import expanduser
from rich.repr import Result
from textual import on
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Horizontal, Container, HorizontalScroll, ScrollableContainer, Vertical, VerticalScroll
from textual.widgets import DirectoryTree, Label, Pretty, Static, Header, Footer, Button, Input, OptionList, RadioButton, RadioSet, TextArea
from typing import Optional
from pathlib import Path

from requester import http
from variables import method
from widgets.data_input import DataInput
from widgets.new_endpoint_modal import NewEndpointModal
from widgets.project_tree import ProjectTree
from widgets.property_dialog import PropertyDialog
from widgets.result import Result

import models.endpoint

class IntroductionApp(App):
    """Introduction to Textual."""
    CSS_PATH = "property_dialog.tcss"

    selected_method: Optional[RadioButton] = None
    selected_file: Optional[DirectoryTree.FileSelected] = None
    selected_directory: Optional[DirectoryTree.DirectorySelected] = None

    BINDINGS = [
        Binding("ctrl+o", "send", "Send", priority=True),
        Binding("ctrl+s", "save", "Save", priority=True),
        Binding("ctrl+n", "new_endpoint", "New Endpoint", priority=True),
        Binding("ctrl+r", "reload_directory", "Reload Directory", priority=True),
    ]

    def compose(self) -> ComposeResult:
        with HorizontalScroll():
            yield ProjectTree(id="project_tree")
            with Vertical():
                yield PropertyDialog(id="property_dialog")
                yield Input(placeholder="URL", id="url")
                with HorizontalScroll():
                    yield DataInput(classes="area")
                    yield Result(id="result", classes="area")
                yield Footer()

    def on_radio_set_changed(self, changed):
        self.selected_method = changed.pressed

    def on_mount(self):
        self.notify(os.path.abspath(expanduser("~/.config/htup")))
        if not os.path.exists(expanduser("~/.config/htup")):
            os.makedirs(expanduser("~/.config/htup"))

    def action_save(self) -> None:
        self.selected_file.path.write_text(
            models.endpoint.Endpoint(
                method = self.selected_method.label._text[0].lower(),
                url = self.query_one("#url").value,
                data = self.query_one("#data_input").text,
            ).model_dump_json()
        )
        self.notify(f"{self.selected_file.path} is saved!")

    def action_send(self) -> None:
        if self.selected_method is None:
            self.notify("メソッドが選択されていないため、自動でGETに設定しました。", severity="warning")
            self.query_one("#radio_get").value = True
            self.selected_method = self.query_one("#radio_get")
            
            
        url = self.query_one("#url").value
        data: TextArea = self.query_one("#data_input")
        res = http.send(url, self.selected_method.label._text[0], data.text)
        if res.status_code == 999:
            self.notify("リクエストが正常に完了しませんでした。",  severity="error")
        self.query_one("#status_code").update(res.status_code)
        self.query_one("#content_type").update(res.content_type)
        self.query_one("#intro").update(res.body)
        self.query_one("#response_time").update(res.time)

    def on_directory_tree_file_selected(self, selected: DirectoryTree.FileSelected):
        self.selected_file = selected
        schema = models.endpoint.load_endpoint(selected.path)
        self.query_one("#url").clear()
        self.query_one("#url").insert_text_at_cursor(schema.url)
        self.query_one(f"#radio_{schema.method}").value = True
        self.query_one("#data_input").text = schema.data

    def on_directory_tree_directory_selected(self, selected: DirectoryTree.DirectorySelected):
        self.selected_directory = selected

    def action_new_endpoint(self) -> None:
        if self.selected_directory is None:
            self.notify("ディレクトリを指定してください。")
        else:
            self.push_screen(NewEndpointModal(self.selected_directory.path, self.selected_directory.node))

    def action_reload_directory(self) -> None:
        self.query_one("#project_directory_tree").reload_node(self.selected_directory.node)

if __name__ == "__main__":
    app = IntroductionApp()
    app.run()

