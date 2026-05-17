# syntax=docker/dockerfile:1
FROM node:26-alpine AS builder

WORKDIR /app

COPY package.json ./
RUN npm install --no-audit --no-fund

COPY . .

ARG API_URL=http://localhost:8080
ENV API_URL=$API_URL

RUN npm run build

FROM nginx:1.31-alpine AS runtime

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
