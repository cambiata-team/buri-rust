# You can pick any Debian/Ubuntu-based image. 😊
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

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

# Install homebrew
RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Add homebrew to path for the developer's user
RUN echo '# Set PATH, MANPATH, etc., for Homebrew.' >> /home/vscode/.profile && \
    echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /home/vscode/.profile && \
    eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

# Add homebrew to root's path for the rest of the Dockerfile
USER root
ENV PATH="/home/linuxbrew/.linuxbrew/bin:${PATH}"

RUN brew install aviator-co/tap/av

# Cargo extensions
USER vscode
RUN cargo install cargo-cranky
RUN cargo install cargo-watch

# Install bun https://bun.sh
RUN curl -fsSL https://bun.sh/install | bash