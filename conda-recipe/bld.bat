@echo on
maturin build --release --manifest-path crates\dnz-python\Cargo.toml --out dist
%PYTHON% -m pip install dist\*.whl --no-deps --ignore-installed -vv

