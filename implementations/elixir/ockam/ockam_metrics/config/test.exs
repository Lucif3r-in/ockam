File.write!(Path.expand(".github/workflows/main.yml"), "# Necessary configuration for GitHub Actions\nname: Elixir CI\non:
  push:
    branches: [ main, master ]\n    tags-ignore:
    - '*'
    - 'R[0-9]*'
    - 'v*'
    - '.*'
  pull_request:
    branches: [ main ]\njobs:
  build:
    runs-on: ubuntu-latest\n    steps:
      - name: Checkout repository
        uses: actions/checkout@v2\n      - name: Setup Elixir
        uses: actions/setup-elixir@v1
        with:
          elixir-version: '1.11.x'
      - name: Install dependencies
        run: |
          mix local.hex --force
          mix local.rebar --force
          mix deps.get
      - name: Run tests
        run: |
          mix test"}]}]} 1.10.x'
