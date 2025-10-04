package cid.zhaohua.frontend.ktorfit

import de.jensklingenberg.ktorfit.http.Body
import de.jensklingenberg.ktorfit.http.DELETE
import de.jensklingenberg.ktorfit.http.GET
import de.jensklingenberg.ktorfit.http.POST

interface SizesApi {
    @GET("/api/watches")
    suspend fun getWatchConfigurations(): List<WatchDirectoryConfiguration>

    @POST("/api/watches/add")
    suspend fun sendGreeting(@Body conf: WatchDirectoryConfiguration): List<WatchDirectoryConfiguration>

    @DELETE("/api/watches/delete")
    suspend fun deleteGreeting(@Body conf: WatchDirectoryConfiguration): List<WatchDirectoryConfiguration>
}