{
  description = "A collection of flake templates";

  outputs = {self}: {
    templates = {
      rust = {
        path = ./rust;
        description = "A Rust project template. Only includes a devShell setup.";
      };

      rust-webapp = {
        path = ./rust-webapp;
        description = "A Rust Axum+Cornucopia+Postgres webapp project template";
      };

      java = {
        path = ./java;
        description = "A Java project template. Only includes a devShell";
      };
    };

    defaultTemplate = self.templates.rust;
  };
}
