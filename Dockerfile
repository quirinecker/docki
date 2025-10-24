FROM nixos/nix AS build

WORKDIR /app

COPY . /app

RUN nix --extra-experimental-features nix-command --extra-experimental-features flakes build \
	&& nix --extra-experimental-features nix-command --extra-experimental-features flakes store gc

RUN mkdir /out && cp result/bin/docki .

