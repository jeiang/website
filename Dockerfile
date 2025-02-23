FROM gradle:jdk21-alpine AS build
COPY --chown=gradle:gradle . /home/gradle/src
WORKDIR /home/gradle/src
RUN gradle build --no-daemon

FROM eclipse-temurin:21-jre-alpine
RUN mkdir /opt/app
COPY --from=build /home/gradle/src/build/libs/ /opt/app/
ENTRYPOINT ["java","-jar","/opt/app/website.jar"]
