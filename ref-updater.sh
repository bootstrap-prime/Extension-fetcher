#! /usr/bin/env nix-shell
#! nix-shell -i bash -p jq

main(){
  api="https://addons.mozilla.org/api/v4/addons/addon"
  extensions="$(jq -r '.[]' extensions.json)"

  for name in $extensions; do

    url="$(curl -s "$api/$name/" | \
      jq -r '.current_version.files[0].url')"

    sha256="$(curl -s "$api/$name/" | \
      jq -r '.current_version.files[0].hash' | \
      sed -E 's/sha256:([a-z0-9A-Z]*)/\1/gm;t;d')"

    echo  "  (pkgs.fetchFirefoxAddon {"
    echo  "    name = \"$name\";"
    echo  "    url = \"$url\";"
    echo  "    sha256 = \"$sha256\";"
    echo  "  })"

  done
}

{
  echo "pkgs: ["
  main
  echo "]"
} | tee sources.nix
