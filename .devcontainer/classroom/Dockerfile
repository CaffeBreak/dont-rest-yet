FROM cffnpwr/git as git


FROM python:3.11

ARG UID=1000
ARG GID=1000
ARG USERNAME=developer

ENV POETRY_HOME=/opt/poetry

WORKDIR /classroom

COPY --from=git /usr/bin/git /usr/bin

RUN apt-get update && \
  apt-get install -y --no-install-recommends sudo && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  groupadd --gid $GID $USERNAME && \
  useradd --uid $UID --gid $GID -m -s /bin/bash $USERNAME && \
  echo "${USERNAME}   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
  wget -O - https://install.python-poetry.org | python3 - && \
  cd /usr/local/bin && \
  ln -s /opt/poetry/bin/poetry && \
  poetry config virtualenvs.create false

USER ${USERNAME}