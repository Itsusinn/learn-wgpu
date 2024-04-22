let
  pkgs = import <nixpkgs> { };

  libraries = with pkgs;[
    wayland
    libxkbcommon
    fontconfig
    libGL
    vulkan-loader
  ];

in
pkgs.mkShell {
  buildInputs = libraries;

  shellHook =
    ''
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    '';
}