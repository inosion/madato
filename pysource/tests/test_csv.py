# csv_file_to_md
# csv_to_md
# spreadsheet_to_csv

from madato import csv_to_md

def test_csv_to_md(artists_csv_str):
    headers = """|  name   |age|genre|albums_sold|hometown |active| label  |"""
    md = csv_to_md(artists_csv_str)
    assert md.startswith(headers)

