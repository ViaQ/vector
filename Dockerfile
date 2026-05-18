FROM registry.redhat.io/ubi9/ubi:latest AS builder

RUN INSTALL_PKGS=" \
      gcc-c++ \
      cmake \
      make \
      git \
      perl \
      openssl-devel \
      llvm-toolset \
      cyrus-sasl \
      llvm \
      cyrus-sasl-devel \
      libtool \
      crypto-policies-scripts \
      " && \
    dnf install -y $INSTALL_PKGS && \
    rpm -V $INSTALL_PKGS && \
    dnf clean all

# Enable post-quantum cryptography
RUN update-crypto-policies --set DEFAULT:PQ

ENV HOME=/root
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.92.0 -y
ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN mkdir -p /src

WORKDIR /src
COPY . /src
RUN /src/scripts/environment/install-protoc.sh
RUN make build

FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN microdnf install -y systemd tar crypto-policies-scripts && \
    microdnf clean all

# Copy PQ crypto-policies configuration from builder
COPY --from=builder /etc/crypto-policies/ /etc/crypto-policies/

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
