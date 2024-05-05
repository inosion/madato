import pytest
import yaml
from madato import yaml_file_to_md, yaml_str_to_md

def test_yaml_file_to_md(toys_yaml_str, tmp_path):
    # Write the YAML data to a temporary file
    p = tmp_path / "toys.yaml"
    with open(p, 'w') as f:
        # write toys_yaml_str
        f.write(toys_yaml_str)

    # Convert the YAML file to Markdown using the Rust function
    md = yaml_file_to_md(str(p))

    print(md)

    # Check that the Markdown is as expected
    assert "|      name      |     type     |color|age_range|price|manufacturer|in_stock|weight|dimensions|safety_certified|country_of_origin|" in md
    assert "|     Doll 🎎     |     Doll     |null |   3-5   | 30  |   Mattel   | false  | 0.5  |12 x 4 x 2|      true      |      China      |" in md

def test_yaml_str_to_md(toys_yaml_str):
    # Convert the YAML data to Markdown using the Rust function
    md = yaml_str_to_md(toys_yaml_str)

    # Check that the Markdown is as expected
    assert "|      name      |     type     |color|age_range|price|manufacturer|in_stock|weight|dimensions|safety_certified|country_of_origin|" in md
    assert "|     Doll 🎎     |     Doll     |null |   3-5   | 30  |   Mattel   | false  | 0.5  |12 x 4 x 2|      true      |      China      |" in md
