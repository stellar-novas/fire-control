let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };

  nativeBuildInputs = with pkgs.buildPackages; [
  	# Rust
  	# rust-bin.stable.latest.default
  	clang
  	trunk
  
  	# misc. libraries
  	openssl
  	pkg-config
  
  	# GUI libs
  	libxkbcommon
  	libGL
  	fontconfig
  
  	# wayland libraries
  	wayland
  
  	# x11 libraries
  	xorg.libXcursor
  	xorg.libXrandr
  	xorg.libXi
  	xorg.libX11
    ];
in
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  inherit nativeBuildInputs;
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath nativeBuildInputs}";
  PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
}


