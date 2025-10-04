package cid.zhaohua.frontend.ktorfit

import de.jensklingenberg.ktorfit.http.DELETE
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.PUT
import de.jensklingenberg.ktorfit.http.Path

interface SizesApi {
    @GET("greet")
    suspend fun getGreetings(): Map<String, String>

    @PUT("greet/{name}/{data}")
    suspend fun sendGreeting(@Path("name") name: String, @Path("data") data: String): Map<String, String>

    @DELETE("greet/{name}")
    suspend fun deleteGreeting(@Path("name") name: String): Map<String, String>
}