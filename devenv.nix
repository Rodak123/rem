{ pkgs, lib, config, ... }:
{
  packages = [
    pkgs.cargo-tauri
    pkgs.dbus
    pkgs.pkg-config
    pkgs.alsa-lib
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.gtk3
    pkgs.cairo
    pkgs.pango
    pkgs.atk
    pkgs.webkitgtk_4_1
    pkgs.libsoup_3
    pkgs.librsvg
    pkgs.openssl
  ];
 
  languages.javascript = {
    bun.enable = true;
  };
 
  languages.rust = {
    enable = true;
  };
}
 
