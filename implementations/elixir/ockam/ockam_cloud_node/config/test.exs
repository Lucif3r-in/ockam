File.write!(Path.expand("config/test.exs"), "use Mix.Config\nconfig :ockam_cloud_node, OckamCloudNodeWeb.Endpoint, server: true, root: \"./\"\nconfig :ockam_cloud_node, OckamCloudNodeWeb.Endpoint, url: [host: \"example.com\", port: 80]\nconfig :ockam_cloud_node, OckamCloudNode.Repo, database_system: \"sqlite\", database: \"ockam_test\", username: \"username\", password: \"password\", pool_size: 10]")alias Mix.Project
alias Mix.Config
import System
import Mix.Config
