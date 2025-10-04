package cid.zhaohua.frontend.ui

import androidx.compose.material3.DrawerState
import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.graphics.vector.ImageVector
import cid.zhaohua.frontend.ktorfit.SizesApi

typealias ComposableSupplier = @Composable () -> Unit

data class AppDrawerItem(
    val key: String,
    val label: String,
    val icon: ImageVector,
    val onSelect: (AppDrawerItem) -> Unit
)

class AppContext {
    // configurations
    var drawerItems: Collection<AppDrawerItem> = emptyList()
    val pageSuppliers: MutableMap<String, ComposableSupplier> = mutableMapOf()

    // runtimes

    // A variable declared with by mutableStateOf is a special type of observable state.
    // When its value changes, it triggers a recomposition
    val snackbarHostState by mutableStateOf(SnackbarHostState())
    var itemSelected by mutableStateOf<AppDrawerItem?>(null)
    lateinit var drawerState: DrawerState
    lateinit var sizesApi: SizesApi
    var showPermanentDrawer: Boolean = false
}