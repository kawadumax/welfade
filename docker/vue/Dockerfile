FROM node:alpine
WORKDIR /vue
RUN apk update && \
    apk add --no-cache git curl && \
    curl -o- -L https://yarnpkg.com/install.sh | sh

ENV PATH $HOME/.yarn/bin:$HOME/.config/yarn/global/node_modules/.bin:$PATH

# ホットリロード
# ENV CHOKIDAR_USEPOLLING=true
RUN yarn global add @vue/cli
RUN yarn install
EXPOSE 8080
CMD ash