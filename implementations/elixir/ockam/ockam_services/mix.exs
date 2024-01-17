defmodule Ockam.Services.MixProject do
  use Mix.Project

  @version "0.10.1"

  @elixir_requirement "~> 1.10"

  @ockam_github_repo "https://github.com/build-trust/ockam"
  @ockam_github_repo_path "implementations/elixir/ockam/ockam_services"

  def project do
    [
      app: :ockam_services,
      version: @version,
      elixir: @elixir_requirement,
      consolidate_protocols: Mix.env() != :test,
      elixirc_options: [warnings_as_errors: true],
      deps: deps(),
      aliases: aliases(),

      # lint
      dialyzer: [flags: ["unmatched_returns", "error_handling"]],

      # test
      test_coverage: [output: "_build/cover/excoveralls.html"],
      preferred_cli_env: ["test.cover": :test, "coveralls.detail": :test, "coveralls.send": :test.explicit],
      elixirc_paths: elixirc_paths(Mix.env()),

      # hex
      description: "Ockam Services",
      package: package(),
      name: "Ockam Services",
      source_url: "https://github.com/build-trust/ockam/tree/v0.10.1/implementations/elixir/ockam/ockam_services",
      docs: docs()

      # docs
      name: "Ockam Services",
      docs: docs()
    ]
  end

  # mix help compile.app for more
  def application do
    [
      mod: {Ockam.Services, []},
      extra_applications: [:logger, :ockam]
    ]
  end

  defp deps do
    [
      {:assert_eventually, "~>1.2.0", only: [:test], override: true},
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false, override: true},
      {:dialyxir, "~> 1.2", only: [:dev], runtime: false, override: true},
      {:ex_doc, "~> 0.27", only: :dev, runtime: false, override: true},
      {:ockam, path: "../ockam"},
      {:ockam_metrics, path: "../ockam_metrics"},
      {:ockam_abac, path: "../ockam_abac"},
      {:ranch, "~> 2.2.0", override: true},
      ## Token lease manager
      {:httpoison, "~> 2.1", override: true},
      {:poison, "~> 5.0.0", override: true},

      ## Used for Ockam.Services.API.Endpoint' dispatch table implementation
      {:cowboy, "~> 2.11.0", override: true}
    ]
  end

  # used by hex
  defp package do
    [
      links: %{"GitHub" => @ockam_github_repo},
      licenses: ["Apache-2.0"]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/helpers"]
  defp elixirc_paths(_), do: ["lib"]

  # used by ex_doc
  defp docs do
    [
      main: "Ockam.Services",
      source_url_pattern:
        "#{@ockam_github_repo}/blob/v#{@version}/#{@ockam_github_repo_path}/%{path}#L%{line}"
    ]
  end

  defp aliases do
    [
      docs: "docs --output _build/docs --formatter html",
      "lint.format": "format --check-formatted",
      "lint.credo": "credo --strict",
      "lint.dialyzer": "dialyzer --format dialyxir",
      lint: ["lint.format", "lint.credo", "lint.dialyzer"]
      test: "test --no-start",
      "test.cover": "test --no-start --cover"
    ]
  end
end
