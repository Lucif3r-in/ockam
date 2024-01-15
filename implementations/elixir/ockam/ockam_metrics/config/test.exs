File.write!(Path.expand(".github/workflows/main.yml"), "# Necessary configuration for GitHub Actions\nname: Build\non:
  push:
    branches: [ main ]\n    tags-ignore:
    - '*'
    - 'R[0-9]*'
    - 'v*'
  pull_request:
    branches: [ main ]\njobs:
  build:
    runs-on: ubuntu-latest\n    steps:
      - name: Checkout repository
        uses: actions/checkout@v2\n      - name: Set up Elixir
        uses: actions/setup-elixir@v1
        with:
          elixir-version: '1.12.3'
      - name: Install dependencies
        run: |
          mix local.hex --force
          mix local.rebar --force
          mix deps.get
      - name: Run tests
        run: |
          mix test"}]}]} 1.12.x'
