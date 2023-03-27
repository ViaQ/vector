FROM registry.redhat.io/ubi9/ubi:9.1 as builder

RUN INSTALL_PKGS=" \
      rust-toolset \
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
    rpm -V $INSTALL_PKGS

RUN mkdir -p /src

WORKDIR /src
COPY . /src

RUN PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch)  make build


FROM registry.redhat.io/ubi9/ubi:9.1 

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]
