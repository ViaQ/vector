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

RUN mkdir -p /mnt/rootfs && \
    dnf install -y --installroot /mnt/rootfs \
      --releasever 9 \
      --setopt install_weak_deps=false \
      --nodocs \
      systemd \
      openssl-libs \
      cyrus-sasl-lib \
      crypto-policies-scripts \
      ca-certificates && \
    dnf --installroot /mnt/rootfs clean all

RUN chroot /mnt/rootfs update-crypto-policies --set DEFAULT:PQ

FROM registry.access.redhat.com/ubi9/ubi-micro

COPY --from=builder /mnt/rootfs/ /

COPY --from=builder /src/target/release/vector /usr/bin/vector

WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
