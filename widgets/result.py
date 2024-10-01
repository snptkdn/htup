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
                yield Label("ResponseBody")
                yield Pretty("Introduction", id="intro", classes="result_label")

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
