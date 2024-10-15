package co.aidanpinard.website

import io.github.oshai.kotlinlogging.KotlinLogging
import io.ktor.server.application.*
import io.ktor.server.application.Application
import io.ktor.server.html.respondHtml
import io.ktor.server.routing.get
import io.ktor.server.routing.routing
import kotlinx.html.*

val logger = KotlinLogging.logger {}

val interests = listOf(
    "Nix/NixOS & Linux",
    "Rust, Go, Zig & Kotlin",
    "Microcontrollers (ESP and Arduino)",
    "Shell Scripting (Bash, Fish, Powershell)"
)
val contacts = listOf(
    Pair("AniList", "https://anilist.co/user/jeiang"),
    Pair("Discord", "https://discordapp.com/users/343517984222085134"),
    Pair("Email", "mailto:aidan@aidanpinard.co"),
    Pair("Github", "https://github.com/jeiang"),
    Pair("Roll20", "https://app.roll20.net/users/6153276/jeiang"),
    Pair("Twitter", "https://twitter.com/jeiang_"),
    Pair("YouTube", "https://www.youtube.com/@jeiang"),
    Pair("Instagram", "https://www.instagram.com/aidanpinard"),
).sortedBy { it.first }

enum class Route(val path: String, val pageTitle: String, val content: FlowContent.() -> Unit) {
    Home("/", "Home", FlowContent::home),
    About("/about", "About", FlowContent::about),
    Contact("/contact", "Contact", FlowContent::contact);

    fun sameRouteOrDefault(path: String): String {
        return if (path == this.path) {
            "#"
        } else {
            path
        }
    }
}

fun Application.routes() {
    for (route in Route.entries) {
        logger.info { "Created ${route.pageTitle} at ${route.path}" }
        routing {
            get(path = route.path) {
                call.pageTemplate(route)
            }
        }
    }
}

suspend inline fun ApplicationCall.pageTemplate(currentPage: Route) =
    respondHtml {
        lang = "en"
        head {
            title {
                +"Aidan Pinard - ${currentPage.pageTitle}"
            }
            // Meta Tags
            meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
            meta(charset = "utf-8")
            meta(name = "author", content = "Aidan Pinard")
            meta(name = "description", content = "Aidan Pinard's personal website.")

            // Fonts
            link(rel = "preconnect", href = "https://fonts.googleapis.com")
            link(rel = "preconnect", href = "https://fonts.gstatic.com") {
                attributes["crossorigin"] = true.toString()
            }
            link(
                href = "https://fonts.googleapis.com/css2?family=Fira+Code&family=Inter:ital,opsz@0,14..32;1,14..32&family=Libre+Baskerville:wght@700&display=swap",
                rel = "stylesheet"
            )

            // Stylesheets
            link(href = "/assets/index.css", rel = "stylesheet", type = "text/css")
        }
        body("flex flex-col leading-relaxed min-h-dvh items-center text-lg text-primary bg-secondary dark:text-secondary dark:bg-primary") {
            header(" sticky z-50 bg-gray-300 top-0 p-4 w-full max-w-xl bg-secondary dark:bg-primary") {
                h1("text-3xl py-4 font-serif font-bold") { +"Aidan Pinard" }
                nav("flex flex-row space-x-3") {
                    for (route in Route.entries) {
                        linkButton(href = currentPage.sameRouteOrDefault(route.path)) {
                            +route.pageTitle
                        }
                    }
                }
            }
            main("flex-grow w-full max-w-xl") {
                currentPage.content(this)
            }
            footer("border-t-4 border-primary dark:border-secondary sticky z-50 bottom-0 p-4 w-full max-w-xl text-center bg-secondary dark:bg-primary") {
                +"Made with Kotlin :)"
            }
        }
    }

inline fun SectioningOrFlowContent.customSection(title: String, crossinline block: SECTION.() -> Unit) =
    section("p-4") {
        h1("text-xl font-serif font-bold") {
            +title
        }
        block()
    }

fun FlowContent.home() {
    customSection("Who I am") {
        p {
            +"My name is Aidan Pinard. I am currently a university student. I'm interested in programming, *nix stuff, and F1. Check out my "
            inlineLink(href = "/about") { +"About page" }
            +" page for more!"
        }
    }
    customSection("Tech I use") {
        ul("list-disc list-inside") {
            interests.map { li { +it } }
        }
    }
}

fun FlowContent.about() {
    customSection("About Me") {
        p("pb-4")  {
            +"I am from Trinidad and have a BSc. in Electrical and Computer Engineering at UWI St. Augustine. I currently work as a software developer at "
            inlineLink(href = "https://cibcfcib.com") { +"CIBC Caribbean" }
            +" and while occasionally doing some personal projects on the side, like this website. Since I started getting into tech, some of the cool things I've learned/used are:"
        }
        ul("list-disc list-inside py-4") {
            li {
                inlineLink(href = "https://nixos.org/") { +"Nix/NixOS" }
                +": Very cool package manager + system management strategy. "
                +"It makes you declare dependencies upfront (e.g. in a flake.nix file) so your packages are 100% reproducible. "
                +"I use NixOS for the server that hosts this site."
            }
            li {
                inlineLink(href = "https://rust-lang.org") { +"Rust" }
                +": Rust is pretty well known at this point, but I like to mention it because it is what really got me to "
                +"understand the value of modelling your domain when writing software."
            }
            li {
                inlineLink(href = "https://ziglang.org/") { +"Zig" }
                +": Zig is probably my favorite language to write, by virtue of its simplicity and flexibility. "
                +"It is still very early as a language, however I am really excited for future improvements."
            }
            li {
                inlineLink(href = "https://kotlinlang.org/") { +"Kotlin" }
                +": Kotlin is a language I started using recently, due to my desire to learn about HTMX, and it seemed "
                +"interesting. The "
                inlineLink(href = "https://github.com/Kotlin/kotlinx.html") { +"Kotlin HTML DSL" }
                +" seemed very interesting, and it was very easy to use."
            }
        }
        p("pb-4") {
            +"Outside of tech, I like watching F1 with my friends (favorite drivers are Alonso and Piastri), and "
            +"playing pool, Balatro, and Crusader Kings 3. I also play F1 2023 and Dirt Rally with a Moza wheel and"
            +" pedal set, though I'm not that good. I also read a lot of manga and light novels, but I haven't watched"
            +"any anime in a good while."
        }
    }
    customSection("About this site") {
        p("pb-4") {
            +"This site was made using :"
        }
        ul("list-disc list-inside pb-4") {
            li { inlineLink(href = "https://kotlinlang.org/") { +"Kotlin" } }
            li { inlineLink(href = "https://ktor.io/") { +"Ktor" } }
            li { inlineLink(href = "https://github.com/Kotlin/kotlinx.html") { +"Kotlin HTML DSL" } }
            li { inlineLink(href = "https://tailwindcss.com/") { +"Tailwind CSS" } }
        }
        p("pb-4") {
            +"And is hosted by Linode."
        }
    }
}

fun FlowContent.contact() {
    customSection("Contact Me!") {
        p("pb-4")  {
            +"You can contact me (or see what I'm up to) via:"
        }
        ul("list-disc list-inside pb-4") { contacts.map { li { inlineLink(href = it.second) { +it.first } } } }
    }
}

inline fun FlowContent.linkButton(
    href: String? = null,
    target: String? = null,
    classes: String = "",
    crossinline block: A.() -> Unit = {}
) = a(href, target, "$classes underline p-2 border-2 rounded border-primary dark:border-secondary", block)

inline fun FlowContent.inlineLink(
    href: String? = null,
    target: String? = null,
    classes: String = "",
    crossinline block: A.() -> Unit = {}
) = a(href, target, "$classes underline", block)

