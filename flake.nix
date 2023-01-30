{
  description = "rust-password generator cli flake";

  outputs = { self, nixpkgs }: {

    packages.x86_64-linux. = nixpkgs.legacyPackages.x86_64-linux.hello;

    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

  };
}
