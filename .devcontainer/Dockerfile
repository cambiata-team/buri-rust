# You can pick any Debian/Ubuntu-based image. 😊
FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

# Options for setup script
ARG INSTALL_ZSH="true"
ARG UPGRADE_PACKAGES="false"
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
# Install needed packages and setup non-root user. Use a separate RUN statement to add your own dependencies.
COPY library-scripts/*.sh /tmp/library-scripts/
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && /bin/bash /tmp/library-scripts/common-debian.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" "true" "true" \
    && apt-get install -y pkg-config zip g++ zlib1g-dev unzip python \
    && apt-get install build-essential curl file git ruby-full locales --no-install-recommends -y \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

# Install Rust toolchain
USER vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN rustup component add rustfmt
RUN cargo install clippy
RUN cargo install cargo-cranky
