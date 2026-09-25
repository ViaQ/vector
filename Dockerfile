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

ARG GIT_COMMIT
RUN export GIT_COMMIT="${GIT_COMMIT:-}"; \
    if [ -z "${GIT_COMMIT}" ] && [ -f .git/HEAD ]; then \
      HEAD_CONTENT=$(cat .git/HEAD | tr -d '\n'); \
      case "${HEAD_CONTENT}" in \
        ref:*) \
          REF_PATH="${HEAD_CONTENT#ref: }"; \
          if [ -f ".git/${REF_PATH}" ]; then \
            GIT_COMMIT=$(cut -c1-10 ".git/${REF_PATH}"); \
          elif [ -f .git/packed-refs ]; then \
            GIT_COMMIT=$(grep " ${REF_PATH}$" .git/packed-refs | cut -c1-10 || true); \
          fi ;; \
        *) \
          GIT_COMMIT=$(echo "${HEAD_CONTENT}" | cut -c1-10) ;; \
      esac; \
      export GIT_COMMIT; \
    fi && \
    PROTOC=/src/thirdparty/protoc/protoc-linux-$(arch) make build

FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN microdnf install -y systemd tar && \
    microdnf clean all

COPY --from=builder /src/target/release/vector /usr/bin
WORKDIR /usr/bin
CMD ["/usr/bin/vector"]

