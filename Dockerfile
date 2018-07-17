FROM node:latest

ADD www-sample /web
WORKDIR /web
RUN npm install 

EXPOSE 8080

CMD npm run serve