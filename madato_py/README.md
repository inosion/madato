# Madato
*Table like data structures to Markdown*

Written in rust (as a lib, and CLI), this library reads various data formats, and writes a ASCII Markdown.

## TL;DR

```
import madato
import yaml
from IPython.display import display, Markdown


my_emoji_yaml = """
  - name: "Rolling on the Floor Laughing"
    character: "🤣"
    unicode: "U+1F923"
  - name: "Face With Tears of Joy"
    character: "😂"
    unicode: "U+1F602"
  - name: "Slightly Smiling Face"
    character: "🙂"
    unicode: "U+1F642"
"""

md_str = madato.yaml_str_to_md(my_emoji_yaml))
print(md_str)
```

gives

```
|            name             |character|unicode|
|-----------------------------|---------|-------|
|Rolling on the Floor Laughing|    🤣    |U+1F923|
|   Face With Tears of Joy    |    😂    |U+1F602|
|    Slightly Smiling Face    |    🙂    |U+1F642|
```

* See the notebook for some more examples [sample_notebook](examples/sample_notebook.ipynb)

## Building

*Make a Wheel*
```
make setup        # creates a venv, installs maturin
maturin develop   # makes a wheel
```

*Run the Python Tests*
```
make setup        # creates a venv, installs maturin
maturin develop   # makes a wheel
pytest
```

