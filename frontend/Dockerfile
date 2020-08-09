FROM node:12-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY ./public/ ./public/

EXPOSE 5000
ENV HOST=0.0.0.0

CMD ["npm", "run", "start"]