FROM registry.redhat.io/ubi9/ubi:latest as builder

RUN INSTALL_PKGS=" \
      gcc-c++ \
      cmake \
      make \
      git \
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
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.72.1 -y
ENV CARGO_HOME=$HOME/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN mkdir -p /src

WORKDIR /src
COPY . /src

RUN PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch) make build

FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN microdnf install -y systemd tar && \
    microdnf clean all

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]

