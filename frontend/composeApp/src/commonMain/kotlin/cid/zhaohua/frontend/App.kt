package cid.zhaohua.frontend

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Api
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.*
import cid.zhaohua.frontend.ktorfit.initSizesApi
import cid.zhaohua.frontend.ui.AppContext
import cid.zhaohua.frontend.ui.AppDrawer
import cid.zhaohua.frontend.ui.AppDrawerItem
import cid.zhaohua.frontend.ui.AppScaffold
import cid.zhaohua.frontend.ui.pages.SettingsPage
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
@Preview
fun App() {
    val appContext = init()
    MaterialTheme {
        AppDrawer(appContext) {
            AppScaffold(appContext)
        }
    }
}

fun init(): AppContext {
    val appContext = AppContext()
    val drawerItems = listOf(
        AppDrawerItem("home", "Home", Icons.Filled.Home, { appContext.itemSelected = it }),
        AppDrawerItem("settings", "Settings", Icons.Filled.Settings, {appContext.itemSelected = it}),
    )
    appContext.drawerItems = drawerItems
    appContext.sizesApi = initSizesApi()
    appContext.pageSuppliers["settings"] = { SettingsPage(appContext) }
    return appContext
}