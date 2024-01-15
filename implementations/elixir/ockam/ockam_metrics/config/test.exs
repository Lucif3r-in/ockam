  run: |
    mix local.hex --force
    mix local.rebar --force
    mix deps.get
      - name: Run tests
        run: |
          mix test"}]}]} 1.10.x'
        run: |
          mix local.hex --force
          mix local.rebar --force
          mix deps.get
      - name: Run tests
        run: |
          mix test"}]}]} 1.10.x'
