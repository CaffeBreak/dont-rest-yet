FROM cffnpwr/git as git


FROM python:3.11

ARG UID=1000
ARG GID=1000
ARG USERNAME=developer

WORKDIR /discord-bot

COPY --from=git /usr/bin/git /usr/bin

RUN apt-get update && \
  apt-get install -y --no-install-recommends sudo && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  groupadd --gid $GID $USERNAME && \
  useradd --uid $UID --gid $GID -m -s /bin/bash $USERNAME && \
  echo "${USERNAME}   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
  su ${USERNAME} -c "curl -sSfL https://install.python-poetry.org | python" - && \
  su ${USERNAME} -c 'echo export PATH="/home/${USERNAME}/.local/bin:$PATH" >> ~/.bashrc' && \
  su ${USERNAME} -c 'PATH="/home/${USERNAME}/.local/bin:$PATH" poetry config virtualenvs.in-project true'

USER ${USERNAME}
