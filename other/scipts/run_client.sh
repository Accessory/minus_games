#!/bin/sh
export LD_LIBRARY_PATH="$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/lib:$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/lib64:/usr/lib:/usr/lib32:/usr/lib64:$HOME/.local/share/lutris/runtime/Ubuntu-18.04-i686:$HOME/.local/share/lutris/runtime/steam/i386/lib/i386-linux-gnu:$HOME/.local/share/lutris/runtime/steam/i386/lib:$HOME/.local/share/lutris/runtime/steam/i386/usr/lib/i386-linux-gnu:$HOME/.local/share/lutris/runtime/steam/i386/usr/lib:$HOME/.local/share/lutris/runtime/Ubuntu-18.04-x86_64:$HOME/.local/share/lutris/runtime/steam/amd64/lib/x86_64-linux-gnu:$HOME/.local/share/lutris/runtime/steam/amd64/lib:$HOME/.local/share/lutris/runtime/steam/amd64/usr/lib/x86_64-linux-gnu:$HOME/.local/share/lutris/runtime/steam/amd64/usr/lib"
export WINEDEBUG="-all"
export DXVK_LOG_LEVEL="none"
export WINEARCH="win64"
export WINE="$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/bin/wine"
export WINE_MONO_CACHE_DIR="$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/mono"
export WINE_GECKO_CACHE_DIR="$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/gecko"
export GST_PLUGIN_SYSTEM_PATH_1_0="$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/lib64/gstreamer-1.0/:$HOME/.local/share/lutris/runners/wine/wine-ge-lol-8-27-x86_64/lib/gstreamer-1.0/"
export WINEPREFIX="$HOME/Games/MinusGames/"
export WINEESYNC="1"
export WINEFSYNC="1"
export DXVK_NVAPIHACK="0"
export DXVK_ENABLE_NVAPI="1"
export WINEDLLOVERRIDES="d3d10core,d3d11,d3d12,d3d12core,d3d9,d3dcompiler_33,d3dcompiler_34,d3dcompiler_35,d3dcompiler_36,d3dcompiler_37,d3dcompiler_38,d3dcompiler_39,d3dcompiler_40,d3dcompiler_41,d3dcompiler_42,d3dcompiler_43,d3dcompiler_46,d3dcompiler_47,d3dx10,d3dx10_33,d3dx10_34,d3dx10_35,d3dx10_36,d3dx10_37,d3dx10_38,d3dx10_39,d3dx10_40,d3dx10_41,d3dx10_42,d3dx10_43,d3dx11_42,d3dx11_43,d3dx9_24,d3dx9_25,d3dx9_26,d3dx9_27,d3dx9_28,d3dx9_29,d3dx9_30,d3dx9_31,d3dx9_32,d3dx9_33,d3dx9_34,d3dx9_35,d3dx9_36,d3dx9_37,d3dx9_38,d3dx9_39,d3dx9_40,d3dx9_41,d3dx9_42,d3dx9_43,dxgi,nvapi,nvapi64=n;winemenubuilder="
export WINE_LARGE_ADDRESS_AWARE="1"
export PYTHONPATH="/usr/lib/lutris:/usr/bin:/usr/lib/python311.zip:/usr/lib/python3.11:/usr/lib/python3.11/lib-dynload:/usr/lib/python3.11/site-packages"

export WINE_EXE=$WINE
export WINE_PREFIX=$WINEPREFIX
export SERVER_URL="http://localhost:8415"
export CLIENT_GAMES_FOLDER="$PWD/target/client_games"
export CLIENT_FOLDER="$PWD/target/client"

cargo run --release --bin minus_games_client -- menu
#printenv