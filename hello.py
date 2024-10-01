import os
from rich.repr import Result
from textual import on
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Horizontal, Container, HorizontalScroll, ScrollableContainer, Vertical, VerticalScroll
from textual.widgets import DirectoryTree, Label, Pretty, Static, Header, Footer, Button, Input, OptionList, RadioButton, RadioSet
from typing import Optional

from requester import http
from widgets.project_tree import ProjectTree
from widgets.property_dialog import PropertyDialog
from widgets.result import Result

class IntroductionApp(App):
    """Introduction to Textual."""
    CSS_PATH = "property_dialog.tcss"

    selected_method: Optional[RadioButton] = None

    BINDINGS = [
        Binding("ctrl+o", "send", "Send", priority=True)
    ]

    def compose(self) -> ComposeResult:
        with HorizontalScroll():
            yield ProjectTree(id="project_tree")
            with Vertical():
                yield PropertyDialog(id="property_dialog")
                yield Input(placeholder="URL", id="url")
                with Container():
                    yield Result(id="result")

    def on_radio_set_changed(self, changed):
        self.selected_method = changed.pressed

    def on_mount(self):
        self.notify(os.path.abspath("~/.config/htup"))
        if not os.path.exists("~/.config/htup"):
            os.makedirs("~/.config/htup")

    def action_send(self) -> None:
        if self.selected_method is None:
            self.notify("メソッドが選択されていないため、自動でGETに設定しました。", severity="warning")
            self.query_one("#radio_get").value = True
            self.selected_method = self.query_one("#radio_get")
            
            
        url = self.query_one("#url").value
        res = http.send(url, self.selected_method.label._text[0])
        if res.status_code == 999:
            self.notify("リクエストが正常に完了しませんでした。",  severity="error")
        self.query_one("#status_code").update(res.status_code)
        self.query_one("#content_type").update(res.content_type)
        self.query_one("#intro").update(res.body)


if __name__ == "__main__":
    app = IntroductionApp()
    app.run()

