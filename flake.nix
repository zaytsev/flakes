{
  description = "A collection of flake templates";

  outputs = { self }: {

    templates = {

      rust = {
        path = ./rust;
        description = "A Rust project template. Only includes a devShell setup.";
      };

			java = {
  			path = ./java;
  			description = "A Java project template. Only includes a devShell";
			};
    };

    defaultTemplate = self.templates.rust;
  };
}
