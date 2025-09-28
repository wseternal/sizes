package cid.zhaohua.frontend

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform