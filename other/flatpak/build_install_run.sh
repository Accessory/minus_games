flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir io.github.accessory.minus_games_client.yml
flatpak build-bundle repo minus_games_client.flatpak io.github.accessory.minus_games_client --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
flatpak remove -y --user io.github.accessory.minus_games_client || true
flatpak install -y --user minus_games_client.flatpak
flatpak run io.github.accessory.minus_games_client