# Build stage
FROM node:lts-alpine AS build
WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile
COPY . .
RUN yarn build

# Serve stage
FROM node:lts-alpine
WORKDIR /app
RUN yarn global add sirv-cli
COPY --from=build /app/build ./build
EXPOSE 3000
CMD ["node", "build"]
