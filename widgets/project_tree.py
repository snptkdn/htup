import os
from os.path import expanduser
from textual.app import ComposeResult
from textual.widgets import DirectoryTree, Static

class ProjectTree(Static):
    def compose(self) -> ComposeResult:
        yield DirectoryTree(path=expanduser("~/.config/htup"), id="project_directory_tree")

    def on_mount(self):
        self.query_one("#project_directory_tree").show_root = False
        self.query_one("#project_directory_tree").show_guides = False
        self.query_one("#project_directory_tree").guide_depth = 1

