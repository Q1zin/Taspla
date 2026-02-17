FROM node:20-alpine

RUN npm install -g bun

WORKDIR /app

COPY package.json bun.lock ./
RUN bun install

COPY . .

EXPOSE 1420
CMD ["bun", "run", "dev", "--host"]
