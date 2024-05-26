from typing import Optional

__version__: str

class KVFilter:
    @property
    def key_re(self) -> str:
        pass

    @property
    def value_re(self) -> str:
        pass


class RenderOptions:
    @property
    def sheet_name(self) -> str:
        pass

    @property
    def headings(self) -> list[str]:
        pass

    @property
    def filters(self) -> list[KVFilter]:
        pass


def yaml_file_to_md(
    filename: str,
    render_options: Optional[RenderOptions] = ...,
) -> str:
    pass


def yaml_str_to_md(
    yaml_str: str,
    render_options: Optional[RenderOptions] = ...,
) -> str:
    pass


def json_file_to_md(
    filename: str,
    render_options: Optional[RenderOptions] = ...,
) -> str:
    pass


def json_str_to_md(json: str, render_options: Optional[RenderOptions] = ...,) -> str:
    pass


def csv_file_to_md(
    filename: str,
    render_options: Optional[RenderOptions] = ...,
) -> str:
    pass


def csv_to_md(csv: str, render_options: Optional[RenderOptions] = ...,) -> str:
    pass


def spreadsheet_to_md(
    filename: str,
    render_options: Optional[RenderOptions] = ...,
) -> str:
    pass


def spreadsheet_to_json(filename: str, sheet_name: Optional[str] = ...,) -> str:
    pass


def spreadsheet_to_yaml(filename: str, sheet_name: Optional[str] = ...,) -> str:
    pass


def spreadsheet_to_csv(filename: str, sheet_name: Optional[str] = ...,) -> str:
    pass