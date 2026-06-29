## e2e tests with lit

1. Create venv (if first time)
```bash
python3 -m venv lit_tests/venv
```

2. Activate venv
```bash
source lit_tests/venv/bin/activate
```

3. Install deps (if first time)
```bash
pip install -r lit_tests/requirements.txt
```

4. Run tests
```bash
cargo test
lit lit_tests
```