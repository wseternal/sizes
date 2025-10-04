package cid.zhaohua.frontend.ktorfit

import de.jensklingenberg.ktorfit.Ktorfit
import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json


fun initSizesApi(): SizesApi {
    val client = HttpClient(CIO) {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                isLenient = true
            })
        }
    }

    // baseUrl must end with "/"
    val ktorfit = Ktorfit.Builder()
        .httpClient(client)
        .baseUrl("http://127.0.0.1:8000/sizes/").build()
    val consoleApi = ktorfit.createSizesApi()
    return consoleApi
}