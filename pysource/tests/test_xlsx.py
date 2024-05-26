import pytest
from pathlib import Path
from madato import spreadsheet_to_md


def test_xlsx_to_md():
    this_dir = Path(__file__).parent
    my_sample_spreadsheet = this_dir / Path("../../test/Financial Sample.xlsx")
    md = spreadsheet_to_md(str(my_sample_spreadsheet))
    headers = "|    Segment     |        Country         | Product |Discount Band|Units Sold|Manufacturing Price|Sale Price|Gross Sales|    Discounts     |       Sales      | COGS  |      Profit      |Date |Month Number|Month Name|Year|"
    assert md.startswith(headers)