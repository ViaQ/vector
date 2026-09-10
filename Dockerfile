FROM registry.redhat.io/ubi9/ubi:latest as builder

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
      " && \
    dnf install -y $INSTALL_PKGS && \
    rpm -V $INSTALL_PKGS && \
    dnf clean all

ENV HOME=/root
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.92.0 -y
ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN mkdir -p /src

WORKDIR /src
COPY . /src

RUN PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch) make build

FROM registry.access.redhat.com/ubi9/ubi:latest AS packages

# Install ubi-micro runtime packages into a staging root directory using DNF
RUN mkdir -p /mnt/rootfs && \
    dnf install -y --installroot=/mnt/rootfs \
      --releasever=9 \
      --setopt=install_weak_deps=false \
      --nodocs \
      systemd \
      tar \
      crypto-policies-scripts && \
    dnf --installroot=/mnt/rootfs clean all

FROM registry.access.redhat.com/ubi9/ubi-micro

COPY --from=packages /mnt/rootfs/ /
# Copy PQ crypto-policies configuration from builder
COPY --from=builder /etc/crypto-policies/ /etc/crypto-policies/
COPY --from=builder /src/target/release/vector /usr/bin/

WORKDIR /usr/bin
CMD ["/usr/bin/vector"]

