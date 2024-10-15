val ktorVersion: String by project
val kotlinVersion: String by project
val logbackVersion: String by project
val klogVersion: String by project
val datetimeVersion: String by project

plugins {
  kotlin("jvm") version "2.0.20"
  id("io.ktor.plugin") version "2.3.12"
  id("org.jetbrains.kotlin.plugin.serialization") version "2.0.10"
}

group = "co.aidanpinard"

version = "1.0"

tasks.register<Exec>("generate-css") {
  doFirst {
    executable("sh")
    args("-c", "tailwindcss -i input.css -o src/main/resources/assets/index.css")
  }
}

tasks.build { dependsOn("generate-css") }

repositories { mavenCentral() }

application {
  mainClass.set("io.ktor.server.netty.EngineMain")

  val isDevelopment: Boolean = project.ext.has("development")
  applicationDefaultJvmArgs = listOf("-Dio.ktor.development=$isDevelopment")
}

dependencies {
  implementation("io.ktor:ktor-server-core-jvm:$ktorVersion")
  implementation("io.ktor:ktor-serialization-kotlinx-json-jvm:$ktorVersion")
  implementation("io.ktor:ktor-server-content-negotiation-jvm:$ktorVersion")
  implementation("io.ktor:ktor-server-resources:$ktorVersion")
  implementation("io.ktor:ktor-server-html-builder:$ktorVersion")
  implementation("io.ktor:ktor-server-auth:$ktorVersion")
  implementation("io.ktor:ktor-server-sessions:$ktorVersion")
  implementation("io.ktor:ktor-server-netty-jvm:$ktorVersion")
  implementation("io.ktor:ktor-server-config-yaml:$ktorVersion")
  implementation("ch.qos.logback:logback-classic:$logbackVersion")
  implementation("io.github.oshai:kotlin-logging-jvm:$klogVersion")
  implementation("org.jetbrains.kotlinx:kotlinx-datetime:$datetimeVersion")
  testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlinVersion")
  testImplementation("io.ktor:ktor-server-test-host-jvm:$ktorVersion")
}

tasks.test { useJUnitPlatform() }

kotlin { jvmToolchain(21) }
