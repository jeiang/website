package co.aidanpinard.website

import io.ktor.http.CacheControl
import io.ktor.server.application.*
import io.ktor.server.http.content.CompressedFileType
import io.ktor.server.http.content.staticResources
import io.ktor.server.routing.routing

fun main(args: Array<String>) {
  io.ktor.server.netty.EngineMain.main(args)
}

fun Application.module() {
  routing {
    staticResources("/assets", "assets") {
      preCompressed(CompressedFileType.GZIP, CompressedFileType.BROTLI)
      enableAutoHeadResponse()
      cacheControl { listOf(CacheControl.MaxAge(maxAgeSeconds = 3600)) }
    }
  }
  routes()
}
