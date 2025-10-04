package cid.zhaohua.frontend.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Apps
import androidx.compose.material3.*
import androidx.compose.material3.MaterialTheme.colorScheme
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch

@Composable
fun AppDrawer(appContext: AppContext, content: ComposableSupplier) {
    appContext.drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)

    var selectedItem by remember { mutableStateOf("") }
    val scope = rememberCoroutineScope()
    val permanentDrawerThreshold = 800.dp

    val drawerContent: @Composable ColumnScope.() -> Unit = {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Icon(Icons.Default.Apps, contentDescription = "App")
            Text("MPConsole", fontWeight = FontWeight.Bold, modifier = Modifier.padding(16.dp))
        }
        HorizontalDivider()
        appContext.drawerItems.forEach {
            NavigationDrawerItem(
                icon = {Icon(it.icon, it.label)},
                label = {
                    Text( it.label)
                },
                modifier = Modifier.padding(8.dp),
                selected = selectedItem == it.key,
                onClick = {
                    scope.launch {
                        selectedItem = it.key
                        appContext.drawerState.close()
                        it.onSelect(it)
                    }
                }
            )
        }
    }

    // show either PermanentNavigationDrawer or ModalNavigationDrawer according to the view width
    BoxWithConstraints(modifier = Modifier.fillMaxSize()) {
        appContext.showPermanentDrawer = this.maxWidth >= permanentDrawerThreshold
        if (appContext.showPermanentDrawer) {
            PermanentNavigationDrawer(
                drawerContent = {
                    PermanentDrawerSheet(
                        modifier = Modifier.width(200.dp),
                        drawerContainerColor = colorScheme.surfaceContainer,
                        content = drawerContent
                    )
                },
                content = content
            )
        } else {
            ModalNavigationDrawer(
                drawerState = appContext.drawerState,
                gesturesEnabled = true,
                drawerContent = {
                    ModalDrawerSheet(
                        modifier = Modifier.width(200.dp),
                        content = drawerContent
                    )
                },
                content = content,
            )
        }
    }
}