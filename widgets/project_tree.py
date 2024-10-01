import os
from textual.app import ComposeResult
from textual.widgets import DirectoryTree, Static

class ProjectTree(Static):
    def compose(self) -> ComposeResult:
        yield DirectoryTree(path="~/.config/htup")

    def on_mount(self):
        self.notify(os.path.abspath("~/.config/htup"))
        if not os.path.exists("~/.config/htup"):
            os.makedirs("~/.config/htup")
