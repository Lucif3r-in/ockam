defmodule OckamMetrics.Config.GitHubActions do
  require Logger

  def create_github_actions_config do
    file_path = Path.expand("config/github_actions.exs")

    case File.open(file_path, [:write]) do
      {:ok, file} ->
        content = "# Correct configuration for GitHub Actions Environment\n# Your GitHub Actions configuration goes here\n# End of GitHub Actions configuration"
        IO.write(file, content)
        File.close(file)
        Logger.info("Created GitHub Actions configuration file")
        :ok
      {:error, reason} ->
        Logger.error("Failed to create GitHub Actions configuration file: #{reason}")
        {:error, reason}
    end
  end
end
