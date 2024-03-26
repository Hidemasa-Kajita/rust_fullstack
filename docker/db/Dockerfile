FROM mysql:8.3.0

RUN apt-get update \
    && apt-get -y install locales-all

ENV MYSQL_ROOT_PASSWORD root
ENV MYSQL_DATABASE app
ENV MYSQL_USER app_user
ENV MYSQL_PASSWORD app_password

ENV TZ=Asia/Tokyo
ENV LANG ja_JP.UTF-8
ENV LANGUAGE ja_JP:ja
ENV LC_ALL ja_JP.UTF-8

COPY ./conf.d/my.cnf /etc/mysql/conf.d/my.cnf
