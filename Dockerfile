FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get install -y \
    git

RUN apt-get update \
  && apt-get install -y tzdata

# https://www.usagi1975.com/201912212117/
RUN apt-get update -qq \
  && apt-get install -y --no-install-recommends \
  sudo curl wget apt-transport-https gnupg \
  ca-certificates  \
  gcc build-essential vim \
  tcl-dev zlib1g-dev

ARG username=my-sqlite
ARG wkdir=/home/my-sqlite

RUN echo "root:root" | chpasswd && \
    adduser --disabled-password --gecos "" "${username}" && \
    echo "${username}:${username}" | chpasswd && \
    echo "%${username}    ALL=(ALL)   NOPASSWD:    ALL" >> /etc/sudoers.d/${username} && \
    chmod 0440 /etc/sudoers.d/${username}

WORKDIR ${wkdir}
RUN chown ${username}:${username} ${wkdir}
USER ${username}

CMD ["bash"]

