from textual import on
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.widgets import Label, Pretty, Static, Header, Footer, Button, Input, OptionList, RadioButton, RadioSet
from typing import Optional

from requester import http

class IntroductionApp(App):
    """Introduction to Textual."""

    selected_method: Optional[RadioButton] = None

    BINDINGS = [
        Binding("ctrl+o", "send", "Send", priority=True)
    ]

    def compose(self) -> ComposeResult:
        with RadioSet(id="method"):
            yield RadioButton("GET", id="radio_get")
            yield RadioButton("POST")
            yield RadioButton("PUT")
            yield RadioButton("DELETE")
        with RadioSet(id="contentType"):
            yield RadioButton("application/json")
        yield Input(placeholder="URL", id="url")
        yield Label("Status Code")
        yield Pretty("-", id="status_code")
        yield Label("Content Type")
        yield Pretty("-", id="content_type")
        yield Label("ResponseBody")
        yield Pretty("Introduction", id="intro")

    def on_radio_set_changed(self, changed):
        self.selected_method = changed.pressed

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

