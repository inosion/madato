#
# This Makefile is only required for the PyO3 python library. It is not required for normal cargo builds.
#

# use Maturin from the docker image
maturin=docker run --rm -v $(PWD):/io ghcr.io/pyo3/maturin

# specify the python version to build for
PYTHON_VERSION=3.10

# specify the target platforms to build for
TARGETS=x86_64-unknown-linux-gnu 
    # x86_64-pc-windows-msvc

#        x86_64-pc-windows-msvc \
#        x86_64-pc-windows-gnu \
#		i686-pc-windows-gnu \
#        \
#		aarch64-apple-darwin \
#		x86_64-apple-darwin \
#        \
#		x86_64-unknown-linux-gnu \
#		aarch64-unknown-linux-gnu \
#		powerpc-unknown-linux-gnu \
#		powerpc64-unknown-linux-gnu \
#        \
#		x86_64-linux-android \
#		aarch64-linux-android \
#		i686-linux-android \
#		\
#		s390x-unknown-linux-gnu \
#		x86_64-sun-solaris \
#		x86_64-unknown-freebsd \

# in make, iterate over all toolchains

toolchains:
	for target in $(TARGETS); do \
		rustup target add $$target; \
	done

prereqs:
	sudo apt install -y llvm

build:
	for target in $(TARGETS); do \
		$(maturin) build --release --target $$target -i python${PYTHON_VERSION}; \
	done

setup:
	python3 -m venv .venv
	.venv/bin/pip install toml
	@# python packaging is soo messy (doing this to avoid cross streams with poetry and maturin)
	.venv/bin/python3 -c 'import toml; c = toml.load("pyproject.toml") ; print("\n".join(c["project"]["optional-dependencies"]["dev"]))' | .venv/bin/pip install -r /dev/stdin

