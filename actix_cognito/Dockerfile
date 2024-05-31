FROM rust:1.70.0
RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install curl gcc vim -y
WORKDIR /project
COPY ./data ./
EXPOSE 80
RUN chmod 777 samplepage
CMD ./samplepage