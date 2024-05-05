setup:
	python3 -m venv .venv
	.venv/bin/pip install toml
	@# pythob packaging is soo messy (doing this to avoid cross streams with poetry and maturin)
	.venv/bin/python3 -c 'import toml; c = toml.load("pyproject.toml") ; print("\n".join(c["project"]["optional-dependencies"]["dev"]))' | .venv/bin/pip install -r /dev/stdin

