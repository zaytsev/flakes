{
  description = "A collection of flake templates";

  outputs = { self }: {

    templates = {

      rust = {
        path = ./rust;
        description = "A Rust project template. Only includes a devShell setup.";
      };

    };

    defaultTemplate = self.templates.rust;
  };
}
