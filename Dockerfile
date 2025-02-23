FROM gradle:jdk21-alpine AS build
WORKDIR /tmp
USER root
RUN apk add --no-cache curl gcompat build-base \
    && curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.4/tailwindcss-linux-arm64 \
    && chmod +x tailwindcss-linux-arm64 \
    && mv tailwindcss-linux-arm64 /bin/tailwindcss \
    && chown gradle:gradle /bin/tailwindcss
USER gradle
COPY --chown=gradle:gradle . /home/gradle/src
WORKDIR /home/gradle/src
RUN gradle build --no-daemon

FROM eclipse-temurin:21-jre-alpine
RUN mkdir /opt/app
COPY --from=build /home/gradle/src/build/libs/ /opt/app/
ENTRYPOINT ["java","-jar","/opt/app/website.jar"]
