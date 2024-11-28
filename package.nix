{
  version,
  lib,
  installShellFiles,
  rustPlatform,
  makeWrapper,
}:
rustPlatform.buildRustPackage {
  pname = "dice";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.difference ./. (
      # don't include in build
      lib.fileset.unions [
        ./README.md
        ./LICENSE
        # ./PKGBUILD
      ]
    );
  };

  inherit version;

  # inject version from nix into the build
  env.NIX_RELEASE_VERSION = version;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    installShellFiles
    makeWrapper
  ];

  postInstall =
    let
      bins = [ "dice" ];
    in
    ''
      for cmd in ${lib.concatStringsSep " " bins}; do
        installShellCompletion --cmd $cmd \
          --bash <($out/bin/$cmd completions bash) \
          --fish <($out/bin/$cmd completions fish) \
          --zsh <($out/bin/$cmd completions zsh)
      done

      installManPage target/man/*
    '';

  postFixup =
    let
      binaries = [ ];
    in
    "wrapProgram $out/bin/dice --prefix PATH : ${lib.makeBinPath binaries}";

  meta = with lib; {
    description = "Simple dice program";
    mainProgram = "dice";
    homepage = "https://github.com/elias-ainsworth/dice";
    license = licenses.mit;
    maintainers = [ maintainers.elias-ainsworth ];
  };
}
