{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
    nativeBuildInputs = with pkgs; [
        alsa-lib    # audio library
    ];

    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
        # X11 libraries
        xorg.libX11
        xorg.libXcursor
        xorg.libxcb
        xorg.libXi
        libxkbcommon

        libGL   # OpenGL library
    ];
}