from textual import on
from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.widgets import Pretty, Static, Header, Footer, Button, Input, OptionList, RadioButton, RadioSet

from requester import http

class IntroductionApp(App):
    """Introduction to Textual."""

    BINDINGS = [
        Binding("ctrl+o", "send", "Send", priority=True)
    ]

    def compose(self) -> ComposeResult:
        with RadioSet():
            yield RadioButton("GET")
            yield RadioButton("POST")
            yield RadioButton("PUT")
            yield RadioButton("DELETE")
        yield Input(placeholder="URL", id="url")
        yield Pretty("Introduction", id="intro")

    def action_send(self) -> None:
        url = self.query_one("#url").value
        res = http.get(url)
        
        self.query_one("#intro").update(res)


if __name__ == "__main__":
    app = IntroductionApp()
    app.run()

